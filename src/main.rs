use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let zip_file = "astroid-2.6.5-py3-none-any.whl";
    let mut archive = zip::ZipArchive::new(std::fs::File::open(zip_file)?)?;

    let dest = Path::new("destination");
    std::fs::create_dir_all(&dest).unwrap();

    // .canonicalize() converts the path to use extendend length syntax on windows
    let dest = dest.canonicalize()?;
    println!("{:?}", dest);

    // This fails when `dest` uses extended length syntax
    archive.extract(dest)?;

    Ok(())
}
