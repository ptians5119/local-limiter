mod local;

static TOTAL_LIMIT: once_cell::sync::OnceCell<i32> = once_cell::sync::OnceCell::new();

/// 初始化限流阈值
pub fn init(limit: i32) {
    TOTAL_LIMIT.set(limit).unwrap();
}

/// 限流
pub async fn limit(key: impl ::std::convert::Into<::std::string::String>,) -> (bool, i32) {
    let count = local::count_by_key(key.into()).await;
    let ceiling = TOTAL_LIMIT.get().cloned().unwrap_or(50);
    (count < ceiling, ceiling - count)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn limit_test() {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let th1 = tokio::spawn(async move {
                for i in 0..15 {
                    let r = limit(123.to_string()).await;
                    println!("1-{}: Response: {:?}", i, r);
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            });
            let th2 = tokio::spawn(async move {
                for i in 0..15 {
                    let r = limit(123.to_string()).await;
                    println!("2-{}: Response: {:?}", i, r);
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            });
            let _ = tokio::join!(th1, th2);
        });
    }

    #[test]
    fn global_test() {
        println!("global_test");
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let key = "1231";
            let cache = super::local::LOCAL_CACHE.lock().await.entry(key.to_string()).or_insert(0).await;
            println!("{:?}", cache.value());
            super::local::LOCAL_CACHE.lock().await.insert(key.to_string(), cache.value() + 1).await;
            // tokio::time::sleep(Duration::from_secs(6)).await;
            let cache = super::local::LOCAL_CACHE.lock().await.entry(key.to_string()).or_insert(0).await;
            println!("{:?}", cache.value());
            super::local::LOCAL_CACHE.lock().await.insert(key.to_string(), cache.value() + 1).await;
            // tokio::time::sleep(Duration::from_secs(6)).await;
            let cache = super::local::LOCAL_CACHE.lock().await.entry(key.to_string()).or_insert(1).await;
            println!("{:?}", cache.value());
        });
    }
}
