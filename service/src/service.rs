use tonic::{transport::Server, Request, Response, Status};
use crate::{
    config::Config,
    service::{
        config::{GetReq, GetResp, SetReq, SetResp},
        limit::{Req, Resp},
        serv::service_server::{Service, ServiceServer}
    }
};

mod serv {
    tonic::include_proto!("service");
}

mod limit {
    tonic::include_proto!("limit");
}

mod config {
    tonic::include_proto!("config");
}

#[derive(Default)]
pub struct GrpcService {}

#[tonic::async_trait]
impl Service for GrpcService {
    async fn limit(
        &self,
        request: Request<Req>,
    ) -> Result<Response<Resp>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = Resp {
            is_ok: true,
            count: 1,
        };
        Ok(Response::new(reply))
    }

    async fn set_config(
        &self,
        request: Request<SetReq>,
    ) -> Result<Response<SetResp>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = SetResp {
            is_ok: true,
            count: 1,
        };
        Ok(Response::new(reply))
    }

    async fn get_config(
        &self,
        request: Request<GetReq>,
    ) -> Result<Response<GetResp>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = GetResp {
            is_ok: true,
            count: 1,
        };
        Ok(Response::new(reply))
    }
}

pub async fn grpc_run(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let addr = config.web.addr.parse().unwrap();
    
    let grpc_server = GrpcService::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(ServiceServer::new(grpc_server))
        .serve(addr)
        .await?;

    Ok(())
}