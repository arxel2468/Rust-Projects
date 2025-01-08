use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::io::{self, Write};
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    println!("Welcome to the File Organizer!");

    println!("Enter the path of the directory you want to organize:");
    io::stdin().read_line(&mut input).unwrap();
    let path = input.trim();

    match organize_files(path) {
        Ok(_) => println!("Files have been successfully organized."),
        Err(e) => println!("Error: {}", e),
    }
}

fn organize_files(directory_path: &str) -> io::Result<()> {
    let path = Path::new(directory_path);

    if !path.is_dir() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Directory not found"));
    }

    let categories = categorize_files();
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let file_path = entry.path();
        
        if file_path.is_file() {
            let extension = match file_path.extension() {
                Some(ext) => ext.to_str().unwrap_or_default(),
                None => continue, // Skip files with no extension
            };

            if let Some(category) = categories.get(extension) {
                let category_dir = path.join(category);
                fs::create_dir_all(&category_dir)?;

                let new_path = category_dir.join(file_path.file_name().unwrap());
                fs::rename(&file_path, new_path)?;
            }
        }
    }
    Ok(())
}

fn categorize_files() -> HashMap<&'static str, &'static str> {
    let mut categories = HashMap::new();
    categories.insert("jpg", "Images");
    categories.insert("png", "Images");
    categories.insert("gif", "Images");
    categories.insert("bmp", "Images");
    categories.insert("txt", "Documents");
    categories.insert("pdf", "Documents");
    categories.insert("docx", "Documents");
    categories.insert("xlsx", "Documents");
    categories.insert("mp4", "Videos");
    categories.insert("mkv", "Videos");
    categories.insert("avi", "Videos");
    categories.insert("mov", "Videos");
    categories.insert("mp3", "Audio");
    categories.insert("wav", "Audio");
    categories.insert("aac", "Audio");
    categories.insert("zip", "Archives");
    categories.insert("tar", "Archives");
    categories.insert("rar", "Archives");
    categories.insert("gz", "Archives");
    categories
}
