use chrono::Timelike;
use moka::future::Cache;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use std::time::Duration;

// todo capacity的默认值
pub static LOCAL_CACHE: Lazy<Mutex<Cache<String, i32>>> = Lazy::new(|| {
    let cache = Cache::builder()
        .time_to_live(Duration::from_secs(60))
        .time_to_idle(Duration::from_secs(1))
        .max_capacity(10000)
        .build();
    Mutex::new(cache)
});

pub async fn count_by_user(id: i64) -> i32 {
    let sec = chrono::Utc::now().second();
    let key = format!("{}{}", id, sec);
    let local_cache = LOCAL_CACHE.lock().await;
    let user_cache = local_cache.entry(key.clone()).or_insert(0).await;

    let count = user_cache.value() + 1;
    local_cache.insert(key, count).await;

    count
}