use colored::*;
use copypasta::ClipboardContext;
use copypasta::ClipboardProvider;
use rand::Rng;

use crate::PassGenConfig;

pub const COPY_PASTA_ERROR: &str = "Could not copy to clipboard.";

pub const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
pub const NUMBERS: &str = "0123456789";
pub const SYMBOLS: &str = ")(*&^%$#@!~";

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

pub fn post_generate_password(config: &PassGenConfig, password: &str) -> Vec<String> {
    let mut output_logs: Vec<String> = vec![];

    if !config.no_copy_to_clipboard {
        let copy_log = self::copy_to_clipboard(&password);
        output_logs.push(copy_log);
    } else {
        output_logs.push(format!("{}", String::from(password).yellow()))
    }

    output_logs
}

pub fn is_numeric(input: &str) -> Result<(), String> {
    let test = &input.parse::<i32>();
    match test {
        Ok(_) => Ok(()),
        Err(__) => Err(String::from("Length must be a number")),
    }
}

pub fn copy_to_clipboard(copy_string: &str) -> String {
    let mut ctx = ClipboardContext::new().expect(self::COPY_PASTA_ERROR);
    let copy_to_clipboard = ctx.set_contents(copy_string.to_string());

    match copy_to_clipboard {
        Ok(_) => format!(
            "{} - {}",
            &copy_string.green(),
            "Copied to clipboard".yellow()
        ),
        Err(__) => format!(
            "{} - {}",
            &copy_string.green(),
            self::COPY_PASTA_ERROR.red()
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_password_length() {
        let config: PassGenConfig = PassGenConfig {
            length: 30,
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
            length: 30,
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
            length: 30,
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
