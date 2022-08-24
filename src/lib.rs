use epub::doc::EpubDoc;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap, fs::File};
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
pub struct JSON<'a>(
    pub HashMap<Cow<'a, str>, HashMap<Cow<'a, str>, Vec<HashMap<Cow<'a, str>, Cow<'a, str>>>>>,
);

pub fn look_up_word(word: &str, language: &str, data: &JSON) -> Option<String> {
    let language = match data.0.get(language) {
        Some(v) => v,
        None => return None,
    };
    let etymologies = match language.get(word) {
        Some(v) => v,
        None => return None,
    };
    let mut return_string = String::new();
    for etymology in etymologies {
        let etymology = etymology;
        let (origin, original_language) = etymology.iter().next().unwrap();
        return_string += &format!(
            "{}'s origin is {} from {}. \n |-{}",
            word,
            origin,
            match_language(original_language),
            look_up_word(origin, original_language, data).unwrap_or("".to_string())
        );
    }
    Some(return_string)
}

fn match_language(lang: &str) -> String {
    match lang {
        "eng" => "English".to_owned(),
        "lat" => "Latin".to_owned(),
        "enm" => "Middle English".to_owned(),
        "ang" => "Old English".to_owned(),
        _ => lang.to_owned(),
    }
}
