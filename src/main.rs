use clap::{App, Arg};
use rand::Rng;
use std::vec::Vec;

fn main() {
    let mut passlen = 8;
    const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
    const NUMBERS: &str = "0123456789";
    const SYMBOLS: &str = ")(*&^%$#@!~";

    let mut charset = String::from(UPPERCASE);
    charset.push_str(LOWERCASE);

    let matches = App::new("randpass")
        .version("0.1")
        .author("Jey")
        .about("Password Generator")
        .args(&[
            Arg::new("length")
                .short('l')
                .about("Length of password")
                .takes_value(true),
            Arg::new("exclude-symbols")
                .short('e')
                .about("Exclude symbols"),
            Arg::new("exclude-numbers")
                .short('n')
                .about("Exclude numbers"),
        ])
        .get_matches();

    if !!!matches.is_present("exclude-symbols") {
        charset.push_str(SYMBOLS);
    }

    if !!!matches.is_present("exclude-numbers") {
        charset.push_str(NUMBERS);
    }

    if let Some(len) = matches.value_of("length") {
        passlen = len.parse().unwrap();
    }

    let char_vec: Vec<char> = charset.chars().collect();

    let mut rng = rand::thread_rng();

    let password: String = (0..passlen)
        .map(|_| {
            let idx = rng.gen_range(0..char_vec.len());
            char_vec[idx] as char
        })
        .collect();

    println!("{:?}", password);
}
