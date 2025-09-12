fn main() {
    tonic_build::configure()
        .compile_protos(
            &["service.proto"],
            &["../libtype/src/proto"],
        )
        .unwrap();
}