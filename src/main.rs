use magic::{Cookie, CookieFlags};
use std::fs::{self, File};
use std::io::{stdin, stdout, Read, Write};
use std::path::PathBuf;
use walkdir::WalkDir;

fn rename_with_extension(path: PathBuf, mut file: File) -> std::io::Result<()> {
    let mut buffer = vec![0; 1024];
    file.read(&mut buffer).expect("Unable to read file");

    let cookie: Cookie = Cookie::open(CookieFlags::MIME_TYPE).unwrap();
    cookie.load::<&str>(&[]).unwrap();
    let mime: String = cookie.buffer(&buffer).unwrap();

    let extension: &str = match mime.as_str() {
        "image/jpeg" => "jpeg",
        "image/png" => "png",
        "image/gif" => "gif",
        "image/tiff" => "tiff",
        "image/bmp" => "bmp",
        "image/webp" => "webp",
        "video/mp4" => "mp4",
        "video/mpeg" => "mpeg",
        "video/quicktime" => "mov",
        "video/x-msvideo" => "avi",
        "video/x-matroska" => "mkv",
        "audio/mpeg" => "mp3",
        "audio/aac" => "aac",
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Unsupported MIME type : {}, {} ",
                    mime,
                    path.file_name()
                        .ok_or_else(|| std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "Invalid file path"
                        ))
                        .map(|name| name.to_string_lossy().to_string())?
                ),
            ))
        }
    };

    let new_path = path.with_extension(extension);
    fs::rename(&path, &new_path)?;

    Ok(())
}

fn main() {
    let mut path = String::new();

    print!("Please enter the absolute path to the directory you want to recover: ");
    stdout().flush().unwrap();

    stdin().read_line(&mut path).expect("Failed to read line");

    if let Some('\n') = path.chars().next_back() {
        path.pop();
    }
    if let Some('\r') = path.chars().next_back() {
        path.pop();
    }

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.into_path();
            if let Ok(file) = File::open(&path) {
                match rename_with_extension(path, file) {
                    Ok(_) => println!("Successfully processed file"),
                    Err(e) => eprintln!("Failed to process file: {}", e),
                }
            }
        }
    }
}
