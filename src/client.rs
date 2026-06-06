pub mod random {
    tonic::include_proto!("random");
}

use random::random_service_client::RandomServiceClient;
use random::RandomRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RandomServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(RandomRequest {
        min: 1,
        max: 100,
    });

    let response = client.get_random_number(request).await?;

    println!("Random number: {}", response.into_inner().number);

    Ok(())
}