use protobuf_codegen_pure as proto;

use std::env;
use std::fs;
use std::path;

const PROTO_MOD_RS: &[u8] = b"
pub mod index;
";

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_dir = path::Path::new(&out_dir);

    let proto_out_dir = path::PathBuf::from(out_dir).join("proto");
    fs::create_dir_all(&proto_out_dir).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=proto/index.proto");
    proto::run(proto::Args {
        out_dir: proto_out_dir.to_str().unwrap(),
        input: &["proto/index.proto"],
        includes: &["proto"],
        customize: proto::Customize {
            ..Default::default()
        },
    })
    .expect("Protobuf code generation failed.");

    fs::write(proto_out_dir.join("mod.rs"), PROTO_MOD_RS).unwrap();
}
