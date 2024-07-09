use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    for proto in ["admin", "systemd", "hwid"].into_iter() {
        let outpath = out_dir.join(format!("{proto}_descriptor.bin"));
        let inpath = format!("api/{proto}/{proto}.proto");

        tonic_build::configure()
            .file_descriptor_set_path(out_dir.join(outpath))
            .compile(&[inpath], &[proto])?;
    }
    Ok(())
}
