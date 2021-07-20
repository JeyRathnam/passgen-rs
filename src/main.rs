mod utils;
use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.2", author = "Jey R. <jairathnem@gmail.com>")]
pub struct PassGenConfig {
    #[clap(short = 'l', long = "length", default_value = "15", validator=utils::is_numeric, about="Length of password")]
    length: i32,

    #[clap(long = "ex-s", about = "Exclude symbols")]
    exclude_symbols: bool,

    #[clap(long = "ex-n", about = "Exclude numbers")]
    exclude_numbers: bool,

    #[clap(long = "nc", about = "Do not copy to clipboard")]
    pub no_copy_to_clipboard: bool,
}

fn main() {
    let options = PassGenConfig::parse();

    let password = utils::generate_password(&options);

    if !options.no_copy_to_clipboard {
        utils::copy_to_clipboard(&password);
    } else {
        println!("{}", &password);
    }
}
