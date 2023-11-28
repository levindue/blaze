pub mod tfidf;
pub mod utils;
pub mod web;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <folder>", args[0]);
        std::process::exit(1);
    }

    let folder = &args[1];
    let files = utils::get_text_files_in_folder(folder);
    let json_name = format!("{folder}.json");

    let index = if utils::should_rebuild_index(folder) {
        tfidf::build_index(&files)
    } else {
        tfidf::load_index(&json_name).expect("Failed to load index")
    };

    tfidf::save_index(&index, &json_name).unwrap();

    web::serve(&files, &index)
}
