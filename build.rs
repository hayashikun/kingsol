use std::fs;
use std::path::Path;

const PROTO_OUT_DIR: &str = "./src/proto";

fn tree(path: &Path, postfix: &str) -> Result<Vec<String>, std::io::Error> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path)? {
        let child = entry?.path();
        let meta = fs::metadata(&child)?;
        if meta.is_dir() {
            files.extend(tree(child.as_path(), postfix)?);
            continue;
        }
        let name = child
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();
        if name.ends_with(postfix) {
            files.push(child.into_os_string().into_string().unwrap());
        }
    }
    return Ok(files);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::remove_dir_all(PROTO_OUT_DIR)?;
    fs::create_dir(PROTO_OUT_DIR).expect("Failed to create out dir");
    fs::write(
        format!("{}/mod.rs", PROTO_OUT_DIR),
        "pub mod kingsol;",
    )?;

    let proto_dir = "./proto";
    let mut files = Vec::new();
    files.extend(tree(&Path::new(proto_dir), ".proto")?);

    let includes = vec![proto_dir];

    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .out_dir(PROTO_OUT_DIR)
        .compile(&files, &includes)?;

    Ok(())
}