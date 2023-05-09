use crate::tfidf::search_index;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use tiny_http::{Header, Response, Server};
use url::Url;

fn url_decode(s: &str) -> String
{
    let mut result = String::new();
    let mut i = 0;
    let bytes = s.as_bytes();

    while i < bytes.len()
    {
        match bytes[i]
        {
            b'%' =>
            {
                let hex = u8::from_str_radix(&s[i + 1..i + 3], 16).unwrap();
                result.push(char::from(hex));
                i += 3;
            }
            _ =>
            {
                result.push(char::from(bytes[i]));
                i += 1;
            }
        }
    }

    result
}

pub fn serve(files: &[String], index: &HashMap<String, HashMap<String, f32>>)
{
    let server = Server::http("0.0.0.0:3435").unwrap();
    println!("Starting server on: http://localhost:3435");

    for request in server.incoming_requests()
    {
        let url = Url::parse(&format!("http://localhost{}", request.url())).unwrap();

        let response = if url.path() == "/"
        {
            let mut file = File::open("index.html").unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let mut response = Response::from_string(contents);
            response.add_header(
                Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap(),
            );
            response.add_header(
                Header::from_bytes(&b"X-Content-Type-Options"[..], &b"nosniff"[..]).unwrap(),
            );
            response
        }
        else if let Some(query) = url.query_pairs().find(|(k, _)| k == "q").map(|(_, v)| v)
        {
            // Search the index with the given query
            let query_words = query.split_whitespace().collect::<Vec<&str>>();
            let results = search_index(&query_words, files, index);
            let json_output = serde_json::to_string(&results).unwrap();
            let mut response = Response::from_string(json_output);
            response.add_header(
                Header::from_bytes(
                    &b"Content-Type"[..],
                    &b"application/json; charset=utf-8"[..],
                )
                .unwrap(),
            );
            response.add_header(
                Header::from_bytes(&b"X-Content-Type-Options"[..], &b"nosniff"[..]).unwrap(),
            );
            response
        }
        else if let Some(path) = url.path().strip_prefix('/')
        {
            let decoded_path = url_decode(path);
            if let Ok(file_content) = std::fs::read_to_string(decoded_path)
            {
                Response::from_string(file_content)
            }
            else
            {
                Response::from_string("File not found".to_string())
            }
        }
        else
        {
            Response::from_string("404".to_string())
        };

        request.respond(response).unwrap();
    }
}
