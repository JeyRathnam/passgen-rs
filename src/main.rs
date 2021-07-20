use clap::{App, Arg};
use utils::PassGenConfig;
mod utils;

fn main() {
    let matches = App::new("passgen-rs")
        .version("0.1")
        .author("Jey <jai.rathnem@gmail.com>")
        .about("Password Generator")
        .args(&[
            Arg::new("length")
                .short('l')
                .long("length")
                .about("Length of password")
                .takes_value(true)
                .validator(utils::is_numeric),
            Arg::new("exclude-symbols")
                .long("ex-s")
                .about("Exclude symbols"),
            Arg::new("exclude-numbers")
                .long("ex-n")
                .about("Exclude numbers"),
            Arg::new("no-copy")
                .long("nc")
                .about("Donot copy to clipboard"),
        ])
        .get_matches();

    let config: PassGenConfig = utils::build_passgen_config(&matches);

    let password = utils::generate_password(&config);

    if !config.no_copy_to_clipboard {
        utils::copy_to_clipboard(&password);
    } else {
        println!("{}", &password);
    }
}
