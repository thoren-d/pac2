use protobuf_codegen_pure as proto;

fn main() {
    println!("cargo:rerun-if-changed=proto/index.proto");
    proto::run(proto::Args {
        out_dir: "src/proto",
        input: &["proto/index.proto"],
        includes: &["proto"],
        customize: proto::Customize {
            ..Default::default()
        },
    })
    .expect("Protobuf code generation failed.");
}
