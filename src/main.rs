use std::env;

use blaze::tfidf::{build_index, save_index};
use blaze::utils::get_files_in_folder;
use blaze::web;

fn main()
{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2
    {
        println!("Usage: {} <command>", args[0]);
        println!("Available commands: help, build, serve");
        std::process::exit(1);
    }

    match args[1].as_str()
    {
        "help" =>
        {
            println!("Usage: {} <command>", args[0]);
            println!("Available commands: help, build, serve");
            std::process::exit(1);
        }

        "build" =>
        {
            if args.len() < 3
            {
                println!("Usage: {} build [folder]", args[0]);
                std::process::exit(1);
            }

            let folder = &args[2];
            let files = get_files_in_folder(folder);
            let index = build_index(&files);
            // TODO: specify file path in args
            save_index(&index, "index.json").unwrap();
        }

        "serve" =>
        {
            let folder = &args[2];
            let files = get_files_in_folder(folder);
            let index = build_index(&files);
            web::serve(&files, &index);
        }

        _ =>
        {
            println!("Invalid command.");
            println!("Available commands: help, build, serve");
            std::process::exit(1);
        }
    }
}
