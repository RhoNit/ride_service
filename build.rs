fn main() {
    tonic_build::compile_protos("proto/ride.proto").unwrap();
}