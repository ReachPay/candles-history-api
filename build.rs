fn main() {
    tonic_build::compile_protos("proto/CandlesHistoryGrpc.proto").unwrap();
}
