use std::io::Read;
use std::{error::Error, fs::File, io::Write, time::Instant};
use walkdir::WalkDir;
use zip::ZipWriter;

pub struct ZipF;

impl ZipF {
    /// ziper a new folder with all its content
    pub fn apply_zip(src: &str, opt: &str) -> Result<(), Box<dyn Error>> {
        let start = Instant::now();

        // Create a new zip archive file
        let mut zip = ZipWriter::new(File::create(opt)?);

        // Walk the directory tree starting from the "src" folder
        for entry in WalkDir::new(src).into_iter().filter_map(|e| e.ok()) {
            // Skip directories
            if entry.file_type().is_dir() {
                continue;
            }

            // Get the path of the file relative to the "src" folder
            let path = entry.path().strip_prefix(src)?.to_string_lossy();

            // Add the file to the zip archive
            let mut file = File::open(entry.path())?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            zip.start_file(path, Default::default())?;
            zip.write_all(&buffer)?;
        }

        // Finish writing the zip archive
        zip.finish()?;

        println!("Zipped {} to {} in {:?}", src, opt, start.elapsed());

        Ok(())
    }
}
