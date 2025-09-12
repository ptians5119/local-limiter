mod local;

/// 限流
pub async fn limit(user_id: i64, _api: impl ::std::convert::Into<::std::string::String>,) -> (bool, i32) {
    let ceiling = 10;
    
    let count = local::count_by_user(user_id).await;
    // println!("Response: {:?}", r);
    (count < ceiling, count)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn limit_test() {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            for i in 0..15 {
                let r = limit(123, "api".to_string()).await;
                println!("{}: Response: {:?}", i, r);
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        });
    }

    #[test]
    fn global_test() {
        println!("global_test");
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let key = "1231";
            let cache = super::local::LOCAL_CACHE.entry(key.to_string()).or_insert(0).await;
            println!("{:?}", cache.value());
            super::local::LOCAL_CACHE.insert(key.to_string(), cache.value() + 1).await;
            // tokio::time::sleep(Duration::from_secs(6)).await;
            let cache = super::local::LOCAL_CACHE.entry(key.to_string()).or_insert(0).await;
            println!("{:?}", cache.value());
            super::local::LOCAL_CACHE.insert(key.to_string(), cache.value() + 1).await;
            // tokio::time::sleep(Duration::from_secs(6)).await;
            let cache = super::local::LOCAL_CACHE.entry(key.to_string()).or_insert(1).await;
            println!("{:?}", cache.value());
        });
    }
}
