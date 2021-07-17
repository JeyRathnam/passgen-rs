use clap::{ ArgMatches};
use rand::Rng;

pub const COPY_PASTA_ERROR: &str = "Could not copy to clipboard.";

pub fn generate_password(matches: &ArgMatches) -> String {
    const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
    const NUMBERS: &str = "0123456789";
    const SYMBOLS: &str = ")(*&^%$#@!~";

    let mut passlen = 8;

    let mut charset = String::from(UPPERCASE);
    charset.push_str(LOWERCASE);

    if !matches.is_present("exclude-symbols") {
        charset.push_str(SYMBOLS);
    }

    if !matches.is_present("exclude-numbers") {
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
    password
}

pub fn is_numeric(input: &str) -> Result<(), String> {
    let test = &input.parse::<u8>();
    match test {
        Ok(_) => Ok(()),
        Err(__) => Err(String::from("Length must be a number")),
    }
}