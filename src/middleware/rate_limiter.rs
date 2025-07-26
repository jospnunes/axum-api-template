use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct RateLimiter {
    store: Arc<Mutex<HashMap<String, (Instant, u32)>>>,
    max_requests: u32,
    window_size: u64,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window_size: u64) -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window_size,
        }
    }

    fn get_rate_limit_key(&self, req: &Request) -> String {
        if let Some(user_agent) = req.headers()
            .get("User-Agent")
            .and_then(|v| v.to_str().ok())
        {
            format!("ua:{}", user_agent)
        } else {
            "unknown".to_string()
        }
    }
}

pub async fn rate_limit_middleware(
    State(limiter): State<RateLimiter>,
    req: Request,
    next: Next,
) -> impl IntoResponse {
    let rate_limit_key = limiter.get_rate_limit_key(&req);
    let now = Instant::now();
    let window = Duration::from_secs(limiter.window_size);

    let exceeded = {
        let mut store = limiter.store.lock().unwrap();

        if let Some(entry) = store.get_mut(&rate_limit_key) {
            if now.duration_since(entry.0) > window {
                *entry = (now, 1);
                false
            } else if entry.1 >= limiter.max_requests {
                true
            } else {
                entry.1 += 1;
                false
            }
        } else {
            store.insert(rate_limit_key, (now, 1));
            false
        }
    };

    if exceeded {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            "Too many requests, try again later",
        ).into_response();
    }

    next.run(req).await
}
