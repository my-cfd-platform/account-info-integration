fn main() {
    tonic_build::compile_protos("proto/TradingInfoIntegrationGrpcService.proto").unwrap();
}
