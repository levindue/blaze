use std::env;

use blaze::tfidf::{build_index, load_index, save_index};
use blaze::utils::{get_txt_files_in_folder, should_rebuild_index};
use blaze::web;

fn main()
{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2
    {
        println!("Usage: {} <folder>", args[0]);
        std::process::exit(1);
    }

    let folder = &args[1];
    let files = get_txt_files_in_folder(folder);
    let json_name = format!("{folder}.json");
    let index;

    if should_rebuild_index(&folder)
    {
        index = build_index(&files);
        save_index(&index, &json_name).unwrap();
    }
    else
    {
        index = load_index(&json_name).unwrap();
    }

    web::serve(&files, &index)
}
