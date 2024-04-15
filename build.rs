use std::{env, path::PathBuf};
fn main() {
    let current_dir = PathBuf::from(env!("ROOT_DIR"));
    let protos_dir: PathBuf = current_dir.join("protos");
    println!("protos directory:{}", protos_dir.display());
    let pwd_proto_file: PathBuf = protos_dir.join("pwd_gen.proto");
    tonic_build::configure()
        .out_dir(&protos_dir)
        .compile(&[pwd_proto_file], &[protos_dir])
        .unwrap();
}
