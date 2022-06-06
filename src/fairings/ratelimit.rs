use crate::utils::error::*;
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{uri::Origin, Method, Status},
    outcome::Outcome,
    Data, Request, Route,
};

use governor::{
    clock::{Clock, DefaultClock},
    state::keyed::DefaultKeyedStateStore,
    Quota, RateLimiter,
};

lazy_static! {
    static ref CLOCK: DefaultClock = DefaultClock::default();
}

pub struct RateLimit {
    limiter: RateLimiter
}

impl RateLimit {
    pub fn new(seconds: u32) -> Self {
        Self {
            limiter: RateLimiter::<String, _, _>::keyed(Quota::per_second(seconds))
        }
    }
}

#[rocket::async_trait]
impl Fairing for RateLimit<'static> {
    fn info(&self) -> Info {
        Info {
            name: "RateLimiter",
            kind: Kind::Request
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        let key = req.real_ip();
        let path = req.uri().path().as_str();

        match self.limiter.check_key(&key) {
            Ok(_) => {
                log::debug!("Allowing {} to access {}", key, path);
            },
            Err(negative) => {
                let wait_time = negative.wait_time_from(CLOCK.now());
                log::debug!("Blocking {} for {} seconds", key, wait_time.as_seconds());
                req.set_method(Method::Get);
                req.set_uri(Origin::parse("/ratelimit").unwrap())
            }
        }
    }
}


#[get("/ratelimit")]
fn rate_limit() -> Status {
    Status::TooManyRequests
}

pub fn routes() -> Vec<Route> {
    routes![rate_limited]
}