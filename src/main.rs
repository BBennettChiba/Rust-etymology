use serde::{Deserialize, Serialize};
// use etymology::open_file;
use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::io;

#[derive(Deserialize, Serialize)]
struct JSON<'a>(HashMap<Cow<'a, str>, HashMap<Cow<'a, str>, Vec<HashMap<Cow<'a, str>, Cow<'a, str>>>>>);

fn main() -> Result<(), Box<dyn Error>> {
    let json = std::fs::read_to_string("src/data.json")?;
    let data = serde_json::from_str(&json).expect("problem with json");
    println!("Please give an english word");
    let mut word = String::new();
    io::stdin().read_line(&mut word)?;
    match look_up_word(word.trim(), "eng", &data) {
        Some(ety) => println!("{}", ety),
        None => println!("No word found"),
    }
    Ok(())
}

fn look_up_word(word: &str, language: &str, data: &JSON) -> Option<String> {
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
