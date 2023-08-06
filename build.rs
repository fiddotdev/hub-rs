fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_files: Vec<_> = std::fs::read_dir("src/protobufs/schemas")?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "proto"))
        .map(|entry| entry.path())
        .filter_map(|path| path.to_str().map(|s| s.to_string()))
        .collect();

    tonic_build::configure()
        .build_server(false)
        .type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]")
        .compile(&proto_files, &["src/protobufs/schemas"])?;

    Ok(())
}
