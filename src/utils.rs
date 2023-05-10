use std::fs;
use std::path::Path;

pub fn get_txt_files_in_folder(dir_path: &str) -> Vec<String>
{
    let mut files = Vec::new();

    for entry in fs::read_dir(dir_path).unwrap().flatten()
    {
        if let Some("txt") = entry.path().extension().and_then(|ext| ext.to_str())
        {
            files.push(entry.path().to_string_lossy().to_string());
        }
    }

    if files.is_empty()
    {
        println!("No text files found in the directory");
        std::process::exit(1);
    }

    files
}

pub fn should_rebuild_index(folder_path: &str) -> bool
{
    let folder_path = Path::new(folder_path);
    let folder_name = folder_path
        .file_name()
        .expect("Failed to get folder name")
        .to_str()
        .expect("Failed to convert folder name to string");

    let json_name = format!("{}.json", folder_name);
    let json_path = Path::new(json_name.as_str());

    if !json_path.exists()
    {
        return true;
    }

    let folder_modified = fs::metadata(&folder_path)
        .expect("Failed to read folder metadata")
        .modified()
        .expect("Failed to get folder modified time");

    let json_modified = fs::metadata(&json_path)
        .expect("Failed to read JSON metadata")
        .modified()
        .expect("Failed to get JSON modified time");

    folder_modified > json_modified
}
