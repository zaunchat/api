use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{uri::Origin, Method, Status},
    Data, Request, Route,
};
use std::{net::IpAddr, time::Duration};

use governor::{
    clock::{Clock, DefaultClock},
    state::keyed::DefaultKeyedStateStore,
    Quota, RateLimiter,
};

lazy_static! {
    static ref CLOCK: DefaultClock = DefaultClock::default();
}

pub struct RateLimit {
    limiter: RateLimiter<IpAddr, DefaultKeyedStateStore<IpAddr>, DefaultClock>,
}

impl RateLimit {
    pub fn new(max: u32, interval: u64) -> Self {
        Self {
            limiter: RateLimiter::<IpAddr, _, _>::keyed(
                Quota::with_period(Duration::from_millis(interval))
                    .unwrap()
                    .allow_burst(max.try_into().unwrap()),
            ),
        }
    }
}

#[rocket::async_trait]
impl Fairing for RateLimit {
    fn info(&self) -> Info {
        Info {
            name: "RateLimiter",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        let key = req.client_ip().expect("Could'nt get client ip");
        let path = req.uri().path().as_str();

        match self.limiter.check_key(&key) {
            Ok(_) => {
                log::debug!("Allowing {} to access {}", key, path);
            }
            Err(negative) => {
                let wait_time = negative.wait_time_from(CLOCK.now());
                log::debug!("Blocking {} for {} seconds", key, wait_time.as_secs());
                req.set_method(Method::Get);
                req.set_uri(Origin::parse("/ratelimit").unwrap())
            }
        }
    }
}

#[get("/ratelimit")]
fn rate_limit() -> Status {
    // TODO: Send rate limit information
    Status::TooManyRequests
}

pub fn routes() -> Vec<Route> {
    routes![rate_limit]
}
