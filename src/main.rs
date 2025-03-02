use gethostname::gethostname;
use std::io::{self, Write};

// mint      #9ff28f (159 242 143)
// lightning #ffcd1c (255 205 28)
// delta     #6F44F0 (111 68  240)
// bossanova #452859 (69  40  89)
// apricot   #f47868 (244 120 104)
#[derive(Debug)]
enum ColorHash {
    Mint,
    Lightning,
    Delta,
    Bossanova,
    Apricot,
}

impl From<u32> for ColorHash {
    fn from(value: u32) -> Self {
        match value % 5u32 {
            0 => ColorHash::Mint,
            1 => ColorHash::Lightning,
            2 => ColorHash::Delta,
            3 => ColorHash::Bossanova,
            _ => ColorHash::Apricot,
        }
    }
}

impl ColorHash {
    fn code(&self) -> String {
        match *self {
            ColorHash::Mint => "\u{001B}[38;2;159;242;143m".to_string(),
            ColorHash::Lightning => "\u{001B}[38;2;255;205;28m".to_string(),
            ColorHash::Delta => "\u{001B}[38;2;111;68;240m".to_string(),
            ColorHash::Bossanova => "\u{001B}[38;2;69;40;89m".to_string(),
            ColorHash::Apricot => "\u{001B}[38;2;244;120;104m".to_string(),
        }
    }
}

// The ColorHash algorithm is:
// "h(k) = sum of [k^2] modulo m",
// Where "k" is each character of the hostname key,
// Converted to a intermediary base36 value, squared, and summed.
// "m" is the size of the hash table
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hostname: String = gethostname()
        .into_string()
        .expect("Error retrieving hostname");

    let characters: Vec<u32> = hostname
        .to_ascii_lowercase()
        .into_bytes()
        .into_iter()
        .map(Into::<u32>::into)
        .collect();

    // - = 45 -> 0
    // 0-9 = 48-57 -> 1-10
    // a-z = 97-122 -> 11-36
    let intermediate: u32 = characters
        .iter()
        .map(|token| {
            let inter_val = match token {
                45 => 0,
                48..=57 => token - 47,
                97..=122 => token - 86,
                _ => 0,
            };
            inter_val.pow(2)
        })
        .sum();

    let mut name_str: String = ColorHash::from(intermediate).code();
    name_str.push_str(&hostname);
    name_str.push_str("\u{001b}[0m");

    print!("{}", name_str);
    io::stdout().flush().unwrap();
    Ok(())
}
