mod grpc_server;

pub mod trading_info_integration_grpc {
    tonic::include_proto!("trading_info_integration");
}

pub use grpc_server::*;