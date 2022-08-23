use epub::doc::EpubDoc;
use serde::{Deserialize, Serialize};
use std::{fs::File, collections::HashMap, borrow::Cow};
use voca_rs::*;


pub fn open_file(path: &str) -> Vec<(String, i32)> {
    let doc = match EpubDoc::new(path) {
        Ok(doc) => doc,
        Err(e) => {
            println!("{}", e);
            return Vec::new();
        }
    };
    let map = map_words_to_count(doc);

    let vec: Vec<(String, i32)> = map.into_iter().map(|(k, v)| (k, v)).collect();
    vec
}

pub fn map_words_to_count(mut doc: EpubDoc<File>) -> std::collections::HashMap<String, i32> {
    let mut map = std::collections::HashMap::new();

    while let Ok(_) = doc.go_next() {
        let strings = strip::strip_tags(&doc.get_current_str().unwrap());
        let split: Vec<&str> = strings.split_whitespace().collect();

        for word in split {
            let count = map.entry(word.to_lowercase()).or_insert(0);
            *count += 1;
        }
    }
    map
}

// fn map_map_to_etymological_roots() {}
#[derive(Deserialize, Serialize)]
pub struct JSON<'a>(pub HashMap<Cow<'a, str>, HashMap<Cow<'a, str>, Vec<HashMap<Cow<'a, str>, Cow<'a, str>>>>>);

