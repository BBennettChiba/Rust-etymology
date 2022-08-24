// use etymology::open_file;
use std::env;
use std::error::Error;
// use std::io::stdin;

// use etymology::{look_up_word, open_file, JSON};
use etymology::open_file;

fn main() -> Result<(), Box<dyn Error>> {
    // let json = std::fs::read_to_string("src/data.json")?;
    // let data: JSON = serde_json::from_str(&json).expect("problem with json");
    // let args: Vec<String> = env::args().collect();
    // let word: String = match args.len() {
    //     1 => {
    //         let mut line = String::new();
    //         println!("we need a word");
    //         let stdin = stdin();
    //         stdin.read_line(&mut line).unwrap();
    //         line
    //     }
    //     _ => args[1].clone(),
    // };
    // match look_up_word(word.trim(), "eng", &data) {
    //     Some(ety) => println!("{}", ety),
    //     None => println!("No word found"),
    // }
    let file = env::args().nth(1).unwrap();
    open_file(&file);
    Ok(())
}
