use std::fs;

pub fn get_files_in_folder(dir_path: &str) -> Vec<String>
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
        println!("No text files found in the directory.");
        std::process::exit(1);
    }

    files
}
