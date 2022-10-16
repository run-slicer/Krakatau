use std::fs;
use std::io::Read;
use std::path::Path;

// pub fn read_files<E>(p: &Path, ext: &str, mut cb: impl FnMut(&[u8]) -> Result<(), E>) -> Result<(), E> {
pub fn read_files(p: &Path, ext: &str, mut cb: impl FnMut(&str, &[u8])) {
    let input_ext = p
        .extension()
        .and_then(|s| s.to_str())
        .expect("Missing input filename extension");
    let input_ext = input_ext.to_ascii_lowercase();

    if input_ext == ext {
        let data = fs::read(p).expect("Error reading input file");
        cb(&p.to_string_lossy(), &data);
    } else if input_ext == "jar" || input_ext == "zip" {
        let mut inbuf = Vec::new();
        let file = fs::File::open(p).expect("Error reading input file");
        let mut zip = zip::ZipArchive::new(file).expect("Error parsing archive");
        let ext = format!(".{}", ext); // temp hack

        for i in 0..zip.len() {
            let mut file = zip.by_index(i).expect("Error parsing archive");
            // println!("found {} {:?} {} {}", i, file.name(), file.size(), file.compressed_size());

            let name = file.name().to_owned();
            if !name.ends_with(&ext) {
                continue;
            }

            inbuf.clear();
            inbuf.reserve(file.size() as usize);
            file.read_to_end(&mut inbuf).unwrap();
            // println!("read {} bytes", inbuf.len());

            cb(&name, &inbuf);
        }
    } else {
        panic!("Unsupported input extension {}", input_ext)
    }
    // Ok(())
}
