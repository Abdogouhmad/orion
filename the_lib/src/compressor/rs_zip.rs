use anyhow::Result;
use std::{
    fs::File,
    io::{Read, Write},
    time::Instant,
};
use tokio::task;
use walkdir::WalkDir;
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

pub struct Zip;

impl Zip {
    /// Zip a folder with all its content in a separate async task
    pub async fn apply_zip(src: &str, opt: &str) -> Result<()> {
        let source = src.to_string();
        let output = opt.to_string();
        // Offload the zipping process to a separate thread
        task::spawn_blocking(move || Zip::zip_folder(&source, &output)).await??;
        Ok(())
    }

    fn zip_folder(src: &String, opt: &String) -> Result<()> {
        let start = Instant::now();

        // Create a new zip archive file
        let zip_file = File::create(opt)?;
        let mut zip = ZipWriter::new(zip_file);

        // Create FileOptions with a specified compression method
        let options = FileOptions::default().compression_method(CompressionMethod::Deflated);

        // Walk the directory tree starting from the "src" folder
        for entry in WalkDir::new(src).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_dir() {
                continue;
            }

            // Get the path of the file relative to the "src" folder
            let path = entry.path().strip_prefix(src)?.to_string_lossy();

            // Open the file and read its contents
            let mut file = File::open(entry.path())?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            // Add the file to the zip archive
            zip.start_file(&*path, options)?;
            zip.write_all(&buffer)?;
        }

        // Finish writing the zip archive
        zip.finish()?;

        println!("Zipped {} to {} in {:?}", src, opt, start.elapsed());
        Ok(())
    }
}
