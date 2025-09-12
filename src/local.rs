use chrono::Timelike;
use moka::future::Cache;
use once_cell::sync::Lazy;
use std::sync::Arc;
use std::time::Duration;

type GlobalCache<K, V> = Arc<Cache<K, V>>;

pub static LOCAL_CACHE: Lazy<GlobalCache<String, i32>> = Lazy::new(|| {
    let cache = Cache::builder()
        .time_to_live(Duration::from_secs(60))
        .time_to_idle(Duration::from_secs(10))
        .max_capacity(10000)
        .build();
    Arc::new(cache)
});

pub async fn count_by_user(id: i64) -> i32 {
    // let sec = chrono::Utc::now().second();
    // let key = format!("{}{}", id, sec);
    // println!("key:{key}");
    let key = id.to_string();
    let user_cache = LOCAL_CACHE.entry(key.clone()).or_insert(0).await;
    
    let count = user_cache.value() + 1;
    LOCAL_CACHE.insert(key, count).await;

    count
}