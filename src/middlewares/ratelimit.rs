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
use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};
lazy_static! {
    static ref CLOCK: DefaultClock = DefaultClock::default();
}

#[derive(Serialize, Clone, Copy, Debug, OpgModel)]
pub struct RateLimitInfo {
    pub limit: u32,
    pub remaining: u32,
    pub retry_after: u64,
}

type RateLimiter =
    Limiter<String, DefaultKeyedStateStore<String>, DefaultClock, StateInformationMiddleware>;

pub async fn ratelimit<B>(
    req: Request<B>,
    next: Next<B>,
    limiter: Arc<RateLimiter>,
) -> Result<Response, Error> {
    let key = if let Some(user) = req.extensions().get::<User>() {
        user.id.to_string()
    } else {
        let header = |name| {
            req.headers()
                .get(name)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.split(',').find_map(|s| s.trim().parse::<IpAddr>().ok()))
        };

        let ip = if *TRUST_CLOUDFLARE && header("CF-Connecting-IP").is_some() {
            header("CF-Connecting-IP")
        } else {
            header("x-forwarded-for")
                .or_else(|| header("x-real-ip"))
                .or_else(|| {
                    req.extensions()
                        .get::<ConnectInfo<SocketAddr>>()
                        .map(|ConnectInfo(addr)| addr.ip())
                })
        };

        ip.expect("Cannot extract IP").to_string()
    };

    let info = match limiter.check_key(&key) {
        Ok(snapshot) => RateLimitInfo {
            limit: snapshot.quota().burst_size().get(),
            remaining: snapshot.remaining_burst_capacity(),
            retry_after: 0,
        },
        Err(negative) => RateLimitInfo {
            limit: negative.quota().burst_size().get(),
            remaining: 0,
            retry_after: negative.wait_time_from(CLOCK.now()).as_secs(),
        },
    };

    if info.retry_after > 0 {
        log::info!("IP: {key} has executed the rate limit");
        return Err(Error::RateLimited(info));
    }

    let mut res = next.run(req).await;
    let headers = res.headers_mut();

    headers.insert("X-RateLimit-Remaining", info.remaining.into());
    headers.insert("X-RateLimit-Limit", info.limit.into());
    headers.insert("Retry-After", info.retry_after.into());

    Ok(res)
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
            $crate::middlewares::ratelimit::ratelimit(req, next, limiter.clone())
        }
    }};
}

pub(crate) use handle;
