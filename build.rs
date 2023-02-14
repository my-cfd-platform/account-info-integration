fn main() {
    tonic_build::compile_protos("proto/AccountInfoIntegrationGrpc.proto").unwrap();
}
