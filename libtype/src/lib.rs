mod entity;
mod logic;

use serv::service_client::ServiceClient;
use anyhow::Result;

// 引入pb文件
mod serv {
    tonic::include_proto!("service");
}
mod limit {
    tonic::include_proto!("limit");
}
mod config {
    tonic::include_proto!("config");
}
use crate::limit::Req;

/// 限流
/// resp: (是否允许， 次数， 剩余时间)
pub async fn limit(
    app: impl ::std::convert::Into<::std::string::String>,
    api: impl ::std::convert::Into<::std::string::String>,
    user_id: Option<i64>) -> Result<entity::api::LimitResp> {
    let mut client = ServiceClient::connect("http://127.0.0.1:50051").await?;
    let request = tonic::Request::new(Req {
        app_name: app.into(),
        user_id: user_id.into(),
        api: api.into(),
    });

    let response = client.limit(request).await?;
    let resp = response.into_inner();

    let r = entity::api::LimitResp {
        is_ok: resp.is_ok,
        count: resp.count as usize,
    };

    println!("Response: {:?}", r);
    Ok(r)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn limit_test() {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let _ = limit("api", "/v1/users", None).await;
        });
    }

    #[test]
    fn global_test() {
        println!("global_test");
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let key = "1231";
            let cache = super::logic::local::LOCAL_CACHE.entry(key.to_string()).or_insert(0).await;
            println!("{:?}", cache.value());
            tokio::time::sleep(Duration::from_secs(6)).await;
            let cache = super::logic::local::LOCAL_CACHE.get(key).await;
            println!("{:?}", cache);
            tokio::time::sleep(Duration::from_secs(6)).await;
            let cache = super::logic::local::LOCAL_CACHE.entry(key.to_string()).or_insert(1).await;
            println!("{:?}", cache.value());
        });
    }
}
