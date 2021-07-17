use clap::{App, Arg};
use copypasta::ClipboardContext;
use copypasta::ClipboardProvider;
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
                .validator(utils::is_numeric),
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

    let password = utils::generate_password(&matches);

    if !matches.is_present("no-copy") {
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


