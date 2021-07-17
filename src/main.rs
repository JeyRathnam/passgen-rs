use clap::{App, Arg, ArgMatches};
use copypasta::ClipboardContext;
use copypasta::ClipboardProvider;
use rand::Rng;
mod utils;

fn main() {
    let matches = App::new("passgen-rs")
        .version("0.1")
        .author("Jey <jai.rathnem@gmail.com>")
        .about("Password Generator")
        .args(&[
            Arg::new("length")
                .short('l')
                .about("Length of password")
                .takes_value(true)
                .validator(is_numeric),
            Arg::new("exclude-symbols")
                .long("es")
                .about("Exclude symbols"),
            Arg::new("exclude-numbers")
                .long("en")
                .about("Exclude numbers"),
            Arg::new("no-copy")
                .long("nc")
                .about("Donot copy to clipboard"),
        ])
        .get_matches();

    let password = generate_password(&matches);

    if !!!matches.is_present("no-copy") {
        let mut ctx = ClipboardContext::new().expect(utils::COPY_PASTA_ERROR);
        let copy_to_clipboard = ctx.set_contents(password.to_string());

        match copy_to_clipboard {
            Ok(_) => println!("{} - Copied to clipboard", &password),
            Err(__) => println!("{}", utils::COPY_PASTA_ERROR),
        }
    } else {
        println!("{}", &password);
    }
}

fn generate_password(matches: &ArgMatches) -> String {
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

fn is_numeric(input: &str) -> Result<(), String> {
    let test = &input.parse::<u8>();
    match test {
        Ok(_) => Ok(()),
        Err(__) => Err(String::from("Length must be a number")),
    }
}
