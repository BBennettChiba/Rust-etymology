// use etymology::open_file;
use std::env;
use std::error::Error;

use etymology::JSON;

fn main() -> Result<(), Box<dyn Error>> {
    let json = std::fs::read_to_string("src/data.json")?;
    let data = serde_json::from_str(&json).expect("problem with json");
    let args: Vec<String> = env::args().collect();
    let word = &args[1];
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
