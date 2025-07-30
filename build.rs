fn main() {
    tonic_build::configure()
        .build_server(true)
        .out_dir("src/") // change output dir if needed
        .compile_protos(
            &["proto/zkp_auth.proto"],
            &["proto/"], // root for imports
        )
        .unwrap();
}
