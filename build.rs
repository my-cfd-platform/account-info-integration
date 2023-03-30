fn main() {
    tonic_build::compile_protos("proto/TradingInfoIntegrationGrpcService.proto").unwrap();
    tonic_build::compile_protos("proto/report_flows_grpc.proto").unwrap();
}
