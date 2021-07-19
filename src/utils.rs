use clap::ArgMatches;
use copypasta::ClipboardContext;
use copypasta::ClipboardProvider;
use rand::Rng;

pub const COPY_PASTA_ERROR: &str = "Could not copy to clipboard.";

pub const DEFAULT_PASSWORD_LENGTH: i32 = 8;

pub const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
pub const NUMBERS: &str = "0123456789";
pub const SYMBOLS: &str = ")(*&^%$#@!~";

pub struct PassGenConfig {
    length: i32,
    exclude_symbols: bool,
    exclude_numbers: bool,
    pub no_copy_to_clipboard: bool,
}

pub fn build_passgen_config(matches: &ArgMatches) -> PassGenConfig {
    let mut config = PassGenConfig {
        length: DEFAULT_PASSWORD_LENGTH,
        exclude_symbols: false,
        exclude_numbers: false,
        no_copy_to_clipboard: false,
    };

    if matches.is_present("exclude-symbols") {
        config.exclude_symbols = true;
    }

    if matches.is_present("exclude-numbers") {
        config.exclude_numbers = true;
    }

    if let Some(len) = matches.value_of("length") {
        config.length = match len.parse() {
            Ok(len) => len,
            Err(_) => DEFAULT_PASSWORD_LENGTH,
        }
    }

    if matches.is_present("no-copy") {
        config.no_copy_to_clipboard = true;
    }

    config
}

pub fn generate_password(config: &PassGenConfig) -> String {
    let passlen = config.length;

    let mut charset = String::from(UPPERCASE);
    charset.push_str(LOWERCASE);

    if !config.exclude_symbols {
        charset.push_str(SYMBOLS);
    }

    if !config.exclude_numbers {
        charset.push_str(NUMBERS);
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
    let test = &input.parse::<i32>();
    match test {
        Ok(_) => Ok(()),
        Err(__) => Err(String::from("Length must be a number")),
    }
}

pub fn copy_to_clipboard(copy_string: &str) {
    let mut ctx = ClipboardContext::new().expect(self::COPY_PASTA_ERROR);
    let copy_to_clipboard = ctx.set_contents(copy_string.to_string());

    match copy_to_clipboard {
        Ok(_) => println!("{} - Copied to clipboard", &copy_string),
        Err(__) => println!("{}", self::COPY_PASTA_ERROR),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_password_length() {
        let config: PassGenConfig = PassGenConfig {
            length: 8,
            exclude_numbers: true,
            exclude_symbols: true,
            no_copy_to_clipboard: true,
        };

        let password = generate_password(&config);
        test_password_length(config.length, &password);
    }

    #[test]
    fn test_generate_password_numbers() {
        let config: PassGenConfig = PassGenConfig {
            length: 8,
            exclude_numbers: false,
            exclude_symbols: true,
            no_copy_to_clipboard: true,
        };

        let password = generate_password(&config);

        assert!(test_password_for_chars(
            NUMBERS,
            &password,
            !config.exclude_numbers
        ));
    }

    #[test]
    fn test_generate_password_symbols() {
        let config: PassGenConfig = PassGenConfig {
            length: 8,
            exclude_numbers: true,
            exclude_symbols: false,
            no_copy_to_clipboard: true,
        };

        let password = generate_password(&config);
        println!("{}", &password);
        assert!(test_password_for_chars(
            SYMBOLS,
            &password,
            !config.exclude_symbols
        ));
    }

    fn test_password_length(length: i32, password: &str) {
        assert_eq!(password.len() as i32, length);
    }

    fn test_password_for_chars(chars: &str, password: &str, should_chars_exist: bool) -> bool {
        if should_chars_exist {
            for c in chars.chars() {
                if password.contains(c) {
                    return true;
                }
            }
            false
        } else {
            for c in chars.chars() {
                if password.contains(c) {
                    return false;
                }
            }
            true
        }
    }
}
