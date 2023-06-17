fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    println!("cargo:rerun-if-changed=proto/threeq.proto");

    prost_build::compile_protos(&["proto/threeq.proto"], &["proto/"])?;

    built::write_built_file().expect("Failed to acquire build-time information");

    Ok(())
}
