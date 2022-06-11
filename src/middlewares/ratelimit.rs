use crate::config::TRUST_CLOUDFLARE;
use crate::structures::User;
use crate::utils::error::Error;
use axum::{extract::ConnectInfo, http::Request, middleware::Next, response::Response};
use governor::{
    clock::{Clock, DefaultClock},
    middleware::StateInformationMiddleware,
    state::keyed::DefaultKeyedStateStore,
    RateLimiter as Limiter,
};
use serde::Serialize;
use std::net::SocketAddr;
use std::sync::Arc;

lazy_static! {
    static ref CLOCK: DefaultClock = DefaultClock::default();
}

#[derive(Serialize, Clone, Copy, Debug, utoipa::Component)]
pub struct RateLimitInfo {
    pub limit: u32,
    pub remaining: u32,
    pub retry_after: u64,
}

type RateLimiter =
    Limiter<String, DefaultKeyedStateStore<String>, DefaultClock, StateInformationMiddleware>;

pub async fn ratelimit<B>(
    mut req: Request<B>,
    next: Next<B>,
    limiter: Arc<RateLimiter>,
) -> Result<Response, Error> {
    let key = if let Some(user) = req.extensions().get::<User>() {
        user.id.to_string()
    } else if *TRUST_CLOUDFLARE {
        req.headers()
            .get("CF-Connecting-IP")
            .and_then(|header| header.to_str().ok())
            .unwrap()
            .to_string()
    } else {
        let addr = req.extensions().get::<ConnectInfo<SocketAddr>>();
        addr.map_or("No IP".to_string(), |i| i.0.ip().to_string())
    };

    let info = match limiter.check_key(&key) {
        Ok(snapshot) => RateLimitInfo {
            limit: snapshot.quota().burst_size().get(),
            remaining: snapshot.remaining_burst_capacity(),
            retry_after: 0,
        },
        Err(negative) => RateLimitInfo {
            limit: negative.quota().burst_size().get(),
            retry_after: negative.wait_time_from(CLOCK.now()).as_secs(),
            remaining: 0,
        },
    };

    if info.retry_after > 0 {
        log::info!("IP: {} has executed the rate limit", key);
        req.extensions_mut().insert(info);
        return Err(Error::RateLimited(info));
    }

    Ok(next.run(req).await)
}

#[macro_export]
macro_rules! handle {
    ($max:expr,$interval:expr) => {{
        let limiter = std::sync::Arc::new(
            governor::RateLimiter::<String, _, _>::keyed(
                governor::Quota::with_period(std::time::Duration::from_millis($interval))
                    .unwrap()
                    .allow_burst($max.try_into().expect("Must be a non-zero number")),
            )
            .with_middleware::<governor::middleware::StateInformationMiddleware>(),
        );

        move |req: axum::http::Request<axum::body::Body>,
              next: axum::middleware::Next<axum::body::Body>| {
            crate::middlewares::ratelimit::ratelimit(req, next, limiter.clone())
        }
    }};
}

pub(crate) use handle;
