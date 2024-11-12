use std::fs;
use std::path::PathBuf;

pub async fn process_list(generation_folder: PathBuf) -> eyre::Result<()> {
    let files = fs::read_dir(generation_folder)?;
    for file in files {
        let path = file?.path();
        // only list files with .bls or .json extension and json files that are valid Addresses
        if let Some(extension) = path.extension() {
            match extension.to_str() {
                Some("bls") => println!("{}", path.display()),
                Some("json") => {
                    if let Some(stem) = path.file_stem() {
                        if let Some(name) = stem.to_str() {
                            if let Ok(_) = alloy::primitives::Address::parse_checksummed(name, None)
                            {
                                println!("{}", path.display());
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}
