use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH };
use std::sync::{ Arc };
use tokio::sync::{ Mutex };

#[derive(Copy, Clone)]
struct Window {
    start: u128,
    count: u32,
}

struct RateLimiter {
    map: HashMap<String, Arc<Mutex<Window>>>,
    window_time_limit: u128,
    window_count_limit: u32,
}

impl RateLimiter {
    fn new(window_time_limit: u128, window_count_limit: u32) -> Self {
        RateLimiter {
            map: HashMap::new(),
            window_time_limit,
            window_count_limit,
        }
    }

    async fn is_allowed(&mut self, user_ip: &str) -> bool {
        let now = SystemTime::now()
          .duration_since(UNIX_EPOCH)
          .expect("Time went backwards");
        let now = now.as_millis();

        if !self.map.contains_key(user_ip) {
            &self.map.insert(user_ip.to_string(), Arc::new(Mutex::new(Window{ start: now, count: 1 })));
            return true;
        }

        let window = self.map.get(user_ip);
        let mut window = &mut window.unwrap().lock().await;
        if now - window.start <= self.window_time_limit {
            if window.count < self.window_count_limit {
                window.count += 1;
                return true;
            } else {
                return false;
            }
        } else {
            window.start = now;
            window.count = 1;
            return true;
        }
    }
}

#[tokio::main]
async fn main() {
    let mut rate_limiter = RateLimiter::new(1000, 2);

    let allowed = rate_limiter.is_allowed("127.0.0.1").await;
    println!("127.0.0.1 allowed: {allowed}");
    let allowed = rate_limiter.is_allowed("127.0.0.1").await;
    println!("127.0.0.1 allowed: {allowed}");
    let allowed = rate_limiter.is_allowed("127.0.0.1").await;
    println!("127.0.0.1 allowed: {allowed}");
    let allowed = rate_limiter.is_allowed("127.0.0.1").await;
    println!("127.0.0.1 allowed: {allowed}");
    let allowed = rate_limiter.is_allowed("127.0.0.1").await;
    println!("127.0.0.1 allowed: {allowed}");
}
