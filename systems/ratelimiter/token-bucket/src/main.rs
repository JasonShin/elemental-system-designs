use std::collections::HashMap;
use std::sync::{Arc};
use std::time::{Duration};
use tokio::{time};
use tokio::sync::{ Mutex };

struct TokenBucket {
    pub capacity: usize,
    pub tokens: usize,
    pub refill_speed: usize,
}

impl TokenBucket {
    fn new(capacity: usize, refill_speed: usize) -> Self {
        TokenBucket {
            capacity,
            tokens: capacity,
            refill_speed,
        }
    }

    async fn start_refill(bucket: &Arc<Mutex<TokenBucket>>) {
        let mut interval = time::interval(Duration::from_millis(bucket.lock().await.refill_speed as u64));
        loop {
            interval.tick().await;
            let mut bucket = bucket.lock().await;
            if bucket.tokens < bucket.capacity {
                bucket.tokens += 1;
            }
        }
    }

    fn add_token(&mut self) {
        if &self.tokens < &self.capacity {
            self.tokens += 1;
        }
    }

    fn take(&mut self) -> bool {
        if self.tokens > 0 {
            self.tokens -= 1;
            return true
        }
        false
    }
}

struct RateLimiter {
    buckets: HashMap<String, Arc<Mutex<TokenBucket>>>,
}

impl RateLimiter {
    fn new() -> Self {
        RateLimiter {
            buckets: HashMap::new(),
        }
    }

    async fn is_allowed(&mut self, ip: &str) -> bool {
        if !self.buckets.contains_key(ip) {
            let token_bucket = Arc::new(Mutex::new(TokenBucket::new(10, 20000)));
            self.buckets.insert(ip.to_string(), token_bucket.clone());
            let refill_bucket = Arc::clone(&token_bucket);
            tokio::spawn(async move {
                TokenBucket::start_refill(&refill_bucket).await;
            });
        }
        let token_bucket = &self.buckets.get(&ip.to_string());
        let mut bucket = &mut token_bucket.unwrap().lock().await;

        // It should either return true or false
        bucket.take()
    }
}

#[tokio::main]
async fn main() {
    let mut rate_limiter = RateLimiter::new();

    loop {
        {
            if rate_limiter.is_allowed("127.0.0.1").await {
                println!("Rate limiter has allowed 127.0.0.1");
            } else {
                println!("Rate limiter is not allowed 127.0.0.1");
            }
        }
        tokio::time::sleep(time::Duration::from_secs(1)).await;
    }

}
