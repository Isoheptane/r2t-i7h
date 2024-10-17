use std::fs::File;
use std::io::Read;
use std::collections::VecDeque;
use std::path::Path;

pub fn i7hize_word(word: &str, threshold: usize) -> Option<String> {
    if word.len() < threshold {
        return Some(String::from(word));
    }
    let (first_char, last_char) = match (word.chars().next(), word.chars().last()) {
        (Some(first), Some(last)) => (first, last),
        _ => { return None; }
    };
    Some(format!("{}{}{}", first_char, word.len() - 2, last_char))
}

pub fn i7hize(text: &str, threshold: usize) -> String {
    let mut result = String::new();
    let mut word = String::new();
    for ch in text.chars() {
        if ch.is_alphabetic() {
            word.push(ch);
            continue;
        }
        if !word.is_empty() {
            result.push_str(&i7hize_word(&word, threshold).unwrap_or_default());
            word.clear();
        }
        result.push(ch);
    }
    result
}

fn main() {
    let mut threshold = 3;
    let mut file_path: Option<String> = None;
    /* Read arguments */
    let mut args: VecDeque<String> = std::env::args().skip(1).collect();
    while let Some(front) = args.pop_front() {
        if vec!["-t", "--threshold"].contains(&front.as_str()) {
            let arg_threshold = args.pop_front()
            .expect(&format!("Threshold required after argument {}", front))
            .parse::<usize>()
            .expect("Failed to parse threshold");
            if arg_threshold < 2 {
                panic!("Illegal threshold");
            }
            threshold = arg_threshold;
            continue;
        }
        file_path = Some(front);
    }
    /* Read text */
    let text = match file_path {
        Some(path) => {
            let mut file = File::open(Path::new(&path)).expect("Failed to open file");
            let mut text = String::new();
            file.read_to_string(&mut text).expect("Failed to read from file");
            text
        },
        None => {
            let mut stdin = std::io::stdin();
            let mut text = String::new();
            stdin.read_to_string(&mut text).expect("Failed to read from stdin");
            text
        }
    };
    println!("{}", i7hize(&text, threshold));
}
