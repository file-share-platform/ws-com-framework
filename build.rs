fn main() {
    println!("cargo:rerun-if-changed=src/message.proto");
    let mut config = prost_build::Config::new();
    config.protoc_arg("--experimental_allow_proto3_optional");
    config
        .compile_protos(&["src/message.proto"], &["src/"])
        .unwrap();
}
