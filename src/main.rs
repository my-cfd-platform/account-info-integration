use std::time::Duration;

use account_info_integration::start_grpc_server;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    start_grpc_server(8888);
    
    loop {
        sleep(Duration::from_secs(100)).await;
    }
}