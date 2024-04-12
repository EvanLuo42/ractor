fn main() -> std::io::Result<()> {
    prost_build::compile_protos(
        &["protos/scenes.proto", "protos/global.proto"],
        &["protos/"]
    )?;
    Ok(())
}