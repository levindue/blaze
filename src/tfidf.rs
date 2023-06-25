use rust_stemmers::{Algorithm, Stemmer};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

fn tokenize(content: &str, stemmer: &Stemmer) -> Vec<String> {
    let mut tokens = Vec::new();

    for word in content.split_whitespace() {
        let clean_word = word.trim_matches(|c: char| !c.is_ascii_alphanumeric());
        if !clean_word.is_empty() {
            tokens.push(stemmer.stem(&clean_word).to_string());
        }
    }

    tokens
}

pub fn build_index(files: &[String]) -> HashMap<String, HashMap<String, f32>> {
    let stemmer = Stemmer::create(Algorithm::English);
    let mut tfidf: HashMap<String, HashMap<String, f32>> = HashMap::new();

    for file_path in files {
        println!("Indexing: {}", file_path);
        let file_content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(_) => continue,
        };

        let mut word_freq = HashMap::new();
        for word in tokenize(&file_content, &stemmer) {
            *word_freq.entry(word).or_insert(0.0) += 1.0;
        }

        let doc_size = word_freq.values().sum::<f32>();
        let mut doc_freq = HashMap::new();
        for (word, freq) in &word_freq {
            doc_freq.insert(word.clone(), freq / doc_size);
        }

        for (word, freq) in &doc_freq {
            let entry = tfidf.entry(word.clone()).or_insert(HashMap::new());
            entry.insert(file_path.clone(), *freq);
        }
    }

    tfidf
}

pub fn save_index(
    index: &HashMap<String, HashMap<String, f32>>,
    file_path: &str,
) -> Result<(), std::io::Error> {
    let json_data = serde_json::to_string(index)?;
    let file = File::create(file_path)?;
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_all(json_data.as_bytes())?;
    buf_writer.flush()?;
    println!("Saved index to: {}", file_path);
    Ok(())
}

pub fn load_index(
    file_path: &str,
) -> Result<HashMap<String, HashMap<String, f32>>, std::io::Error> {
    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);
    let mut json_data = String::new();
    buf_reader.read_to_string(&mut json_data)?;
    let index: HashMap<String, HashMap<String, f32>> = serde_json::from_str(&json_data)?;
    Ok(index)
}

pub fn search_index(
    query: &[&str],
    files: &[String],
    tfidf: &HashMap<String, HashMap<String, f32>>,
) -> Vec<(String, f32)> {
    let mut file_scores = Vec::new();

    let stemmer = Stemmer::create(Algorithm::English);

    let mut stemmed_query: HashSet<String> = HashSet::new();
    for term in query {
        let stemmed = stemmer.stem(term);
        stemmed_query.insert(stemmed.to_string());
    }

    for file_path in files {
        match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(_) => continue,
        };

        let mut score = 0.0;
        let mut doc_freq = HashSet::new();
        for word in &stemmed_query {
            if !tfidf.contains_key(word) {
                continue;
            }
            doc_freq.insert(word);
            if let Some(w_freq) = tfidf.get(word).unwrap().get(file_path) {
                score += w_freq * tfidf.len() as f32 / doc_freq.len() as f32;
            }
        }
        file_scores.push((file_path.clone(), score));
    }

    file_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    file_scores.clone()
}
