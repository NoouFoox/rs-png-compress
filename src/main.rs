use std::path::Path;
use walkdir::WalkDir;
use oxipng::{optimize, InFile, Options as OxOptions, OutFile};

fn main() {
    let dirs = WalkDir::new("/Users/junhao/Downloads/textures");
    for dir in dirs {
        let dir = dir.unwrap();
        let path = dir.path();
        match path.extension() {
            Some(ext) if ext.to_str() == Some("png") => png_compress(path),
            _ => {}
        }
    }
}

fn png_compress(path: &Path) {
    let mut options = OxOptions::from_preset(6);
    options.optimize_alpha = true;
    options.interlace = Some(oxipng::Interlacing::None);
    let in_file = InFile::Path(path.to_path_buf());
    let out_file = OutFile::Path { path:Some(path.to_path_buf()), preserve_attrs: false };

    match optimize(&in_file, &out_file, &options) {
        Ok(_) => println!("PNG compression successful: {}", path.display()),
        Err(e) => println!("PNG compression failed: {}", e),
    }
}