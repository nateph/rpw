use clap::Clap;
use rand::Rng;

const ALPHABET_UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ALPHABET_LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &str = "0123456789";
const SYMBOLS: &str = "~!@#$%^&*()-+=[]";
/// Generate a random password
#[derive(Clap)]
#[clap(version)]
struct Options {
    /// Do not include lower case characters
    #[clap(short = 'l', long = "no-lower")]
    no_lowercase: bool,
    /// Do not include uppercase case characters
    #[clap(short = 'u', long = "no-upper")]
    no_uppercase: bool,
    /// Do not include symbol characters
    #[clap(short = 's', long = "no-symbols")]
    no_symbols: bool,
    ///  Do not include numbers
    #[clap(short = 'n', long = "no-numbers")]
    no_numbers: bool,
    /// Specify a custom length (max 255)
    #[clap(long = "len", default_value = "16")]
    length: u8,
    /// Exclude certain characters explicitly. example: --exlclude '4$&'
    #[clap(long)]
    exclude: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::parse();

    let mut valid_chars = String::new();
    if !options.no_lowercase {
        valid_chars.push_str(ALPHABET_LOWER);
    }
    if !options.no_numbers {
        valid_chars.push_str(NUMBERS);
    }
    if !options.no_symbols {
        valid_chars.push_str(SYMBOLS);
    }
    if !options.no_uppercase {
        valid_chars.push_str(ALPHABET_UPPER);
    }
    if let Some(s) = options.exclude {
        valid_chars.retain(|c| !s.as_str().contains(c));
    }

    let valid_chars = valid_chars.as_bytes();
    let mut rng = rand::thread_rng();
    let password: String = (0..options.length)
        .map(|_| {
            let c = rng.gen_range(0..valid_chars.len());
            valid_chars[c] as char
        })
        .collect();

    println!("{}", password);

    Ok(())
}
