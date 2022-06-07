use crate::structures::User;
use governor::{
    clock::{Clock, DefaultClock},
    middleware::StateInformationMiddleware,
    state::keyed::DefaultKeyedStateStore,
    Quota, RateLimiter as Limiter,
};
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{uri::Origin, ContentType, Method, Status},
    outcome::Outcome,
    Data, Request, Response, Route,
};
use serde_json::json;
use std::{
    io::Cursor,
    ops::Add,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use dashmap::{mapref::one::Ref, DashMap};

type SharedRateLimiter =
    Limiter<String, DefaultKeyedStateStore<String>, DefaultClock, StateInformationMiddleware>;

lazy_static! {
    static ref CLOCK: DefaultClock = DefaultClock::default();
    static ref MAP: DashMap<String, SharedRateLimiter> = DashMap::new();
}

pub struct RateLimiter;

fn limit_of(key: &String) -> (u32, u64) {
    match key.as_str() {
        "/auth" => (10, 1000 * 60 * 60 * 3),
        "/channels" => (15, 1000 * 5),
        "/users" => (20, 1000 * 5),
        "/servers" => (10, 1000 * 5),
        _ => (50, 1000 * 60 * 1),
    }
}

impl RateLimiter {
    fn of_route(&self, req: &Request<'_>) -> Ref<String, SharedRateLimiter> {
        let key = "/".to_owned() + req.routed_segment(0).unwrap_or_default();

        if let Some(value) = MAP.get(&key) {
            return value;
        } else {
            let (max, interval) = limit_of(&key);
            let limiter = Limiter::<String, _, _>::keyed(
                Quota::with_period(Duration::from_millis(interval))
                    .unwrap()
                    .allow_burst(max.try_into().expect("Must be a non-zero number")),
            )
            .with_middleware::<StateInformationMiddleware>();
            MAP.insert(key.clone(), limiter);
        }

        MAP.get(&key).unwrap()
    }
}

fn now() -> Duration {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
}

#[derive(Clone, Copy)]
struct RateLimitInfo {
    pub limit: u32,
    pub remaining: u32,
    pub retry_after: u64,
    pub reset: u64,
}

impl RateLimiter {
    async fn check(&self, req: &Request<'_>) -> RateLimitInfo {
        *req.local_cache_async(async {
            let key = if let Outcome::Success(user) = req.guard::<User>().await {
                user.id.to_string()
            } else {
                req.remote().map(|x| x.ip().to_string()).unwrap_or_default()
            };

            let limiter = self.of_route(req);

            match limiter.check_key(&key) {
                Ok(snapshot) => RateLimitInfo {
                    limit: snapshot.quota().burst_size().get(),
                    remaining: snapshot.remaining_burst_capacity(),
                    retry_after: 0,
                    reset: 0,
                },
                Err(negative) => RateLimitInfo {
                    limit: negative.quota().burst_size().get(),
                    retry_after: negative.wait_time_from(CLOCK.now()).as_secs(),
                    reset: now().add(negative.wait_time_from(CLOCK.now())).as_millis() as u64,
                    remaining: 0,
                },
            }
        })
        .await
    }
}

#[rocket::async_trait]
impl Fairing for RateLimiter {
    fn info(&self) -> Info {
        Info {
            name: "RateLimiter",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        let info = self.check(req).await;

        if info.remaining == 0 {
            req.set_method(Method::Get);
            req.set_uri(Origin::parse("/ratelimit").unwrap())
        }
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        let RateLimitInfo {
            remaining,
            retry_after,
            limit,
            reset,
        } = self.check(req).await;

        res.set_raw_header("X-RateLimit-Limit", limit.to_string());
        res.set_raw_header("X-RateLimit-Remaining", remaining.to_string());
        res.set_raw_header("X-RateLimit-After", retry_after.to_string());
        res.set_raw_header("X-RateLimit-Reset", reset.to_string());

        if remaining == 0 {
            let body = json!({
                "limit": limit,
                "remaining": remaining,
                "reset": reset,
                "retry_after": retry_after,
            })
            .to_string();
            res.set_header(ContentType::JSON);
            res.set_sized_body(body.len(), Cursor::new(body));
        }
    }
}

#[get("/ratelimit")]
fn rate_limit() -> Status {
    Status::TooManyRequests
}

pub fn routes() -> Vec<Route> {
    routes![rate_limit]
}
