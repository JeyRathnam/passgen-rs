mod utils;
use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(version = "0.2", author = "Jey R. <jairathnem@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
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
    let config = PassGenConfig::parse();

    let password = utils::generate_password(&config);

    let output_logs = utils::post_generate_password(&config, &password);

    for log in output_logs {
        println!("{}", log);
    }
}
