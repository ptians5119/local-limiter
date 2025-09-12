use chrono::Timelike;
use moka::future::Cache;
use once_cell::sync::Lazy;
use std::sync::Arc;
use std::time::Duration;

type GlobalCache<K, V> = Arc<Cache<K, V>>;

pub static LOCAL_CACHE: Lazy<GlobalCache<String, i32>> = Lazy::new(|| {
    let cache = Cache::builder()
        .time_to_live(Duration::from_secs(2))
        .time_to_idle(Duration::from_secs(1))
        .max_capacity(10000)
        .build();
    Arc::new(cache)
});

pub async fn limit_by_user(id: i64, api: String) -> i32 {
    let sec = chrono::Utc::now().second();
    let key = format!("{}{}", id, sec);
    let user_cache = LOCAL_CACHE.entry(key.clone()).or_insert(0).await;
    
    let count = user_cache.into_value() + 1;
    LOCAL_CACHE.insert(key, count).await;

    count
}