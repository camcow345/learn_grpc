use rand::Rng;
use tonic::{transport::Server, Request, Response, Status};

pub mod random {
    tonic::include_proto!("random");
}

use random::random_service_server::{RandomService, RandomServiceServer};
use random::{RandomRequest, RandomResponse};

#[derive(Default)]
pub struct MyRandomService;

#[tonic::async_trait]
impl RandomService for MyRandomService {
    async fn get_random_number(
        &self,
        request: Request<RandomRequest>,
    ) -> Result<Response<RandomResponse>, Status> {
        let req = request.into_inner();

        if req.min > req.max {
            return Err(Status::invalid_argument("min cannot be greater than max"));
        }

        let number = rand::rng().random_range(req.min..=req.max);

        Ok(Response::new(RandomResponse { number }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = MyRandomService::default();

    println!("Random gRPC server listening on {}", addr);

    Server::builder()
        .add_service(RandomServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}