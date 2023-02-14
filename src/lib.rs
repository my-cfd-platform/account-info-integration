mod grpc_server;

pub mod account_info_integration_grpc {
    tonic::include_proto!("account_info_integration");
}

pub use grpc_server::*;