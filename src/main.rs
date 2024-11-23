use colored::*;
use std::fs::{read_to_string, write};
use std::io::{self, Write};

struct Tapcode {
    grid: Vec<Vec<char>>,
    tap_symbol: char,
}

impl Tapcode {
    fn new(alphabet: &str, tap_symbol: char) -> Result<Self, &'static str> {
        if alphabet.len() != 25 {
            return Err("Alphabet must contain exactly 25 characters.");
        }

        let grid = alphabet
            .to_lowercase()
            .chars()
            .collect::<Vec<char>>()
            .chunks(5)
            .map(|chunk| chunk.to_vec())
            .collect();

        Ok(Self { grid, tap_symbol })
    }

    fn encode(&self, message: &str) -> Result<String, &'static str> {
        if message.trim().is_empty() {
            return Err("Message cannot be empty.");
        }

        let mut encoded_message = String::new();
        for ch in message.to_lowercase().chars() {
            if let Some((row, col)) = self.find_char(ch) {
                let taps = format!(
                    "{} {} ",
                    self.tap_symbol.to_string().repeat(row + 1),
                    self.tap_symbol.to_string().repeat(col + 1)
                );
                encoded_message.push_str(&taps);
            } else if ch.is_whitespace() {
                encoded_message.push('|');
            } else {
                return Err("Message contains characters not in the grid.");
            }
        }
        Ok(encoded_message.trim_end().to_string())
    }

    fn decode(&self, code: &str) -> Result<String, &'static str> {
        if code.trim().is_empty() {
            return Err("Tapcode input cannot be empty.");
        }

        let mut decoded_message = String::new();
        let words = code.split('|');
        for word in words {
            let taps: Vec<&str> = word.trim().split_whitespace().collect();
            if taps.len() % 2 != 0 {
                return Err("Invalid Tapcode input.");
            }
            for chunk in taps.chunks(2) {
                let row = chunk[0].len() - 1;
                let col = chunk[1].len() - 1;
                if let Some(&ch) = self.grid.get(row).and_then(|r| r.get(col)) {
                    decoded_message.push(ch);
                } else {
                    return Err("Invalid Tapcode coordinates.");
                }
            }
            decoded_message.push(' ');
        }
        Ok(decoded_message.trim_end().to_string())
    }

    fn find_char(&self, ch: char) -> Option<(usize, usize)> {
        for (row_idx, row) in self.grid.iter().enumerate() {
            if let Some(col_idx) = row.iter().position(|&c| c == ch) {
                return Some((row_idx, col_idx));
            }
        }
        None
    }
}

fn display_help() {
    println!("{}", "Tapcode Cipher Help".bold().underline());
    println!("\nFeatures:");
    println!("1. Encode: Convert a message into Tapcode using the current grid.");
    println!("2. Decode: Convert Tapcode back into a message using the current grid.");
    println!("3. Set Custom Grid: Define a custom 5x5 grid for encoding/decoding.");
    println!("4. Save Grid: Save the current grid to a file.");
    println!("5. Load Grid: Load a grid from a file.");
    println!("6. Exit: Quit the program.\n");

    println!("Tapcode Notes:");
    println!("  - Separate words in Tapcode using '|'.");
    println!("  - Messages must only contain characters from the grid.\n");
}

fn save_grid_to_file(tapcode: &Tapcode) {
    let grid_string: String = tapcode
        .grid
        .iter()
        .flatten()
        .collect::<Vec<&char>>()
        .iter()
        .map(|&&c| c)
        .collect();

    if let Err(e) = write("tapcode_grid.txt", grid_string) {
        println!("Error saving grid to file: {}", e);
    } else {
        println!("{}", "Grid saved to tapcode_grid.txt!".green());
    }
}

fn load_grid_from_file() -> Result<String, &'static str> {
    match read_to_string("tapcode_grid.txt") {
        Ok(content) => {
            if content.len() == 25 {
                Ok(content)
            } else {
                Err("Grid file must contain exactly 25 characters.")
            }
        }
        Err(_) => Err("Could not read grid file. Ensure it exists and is accessible."),
    }
}

fn main() {
    println!("{}", "Tapcode Cipher".bold().cyan());
    println!("1. Encode a message");
    println!("2. Decode a Tapcode");
    println!("3. Set a custom 5x5 alphabet grid");
    println!("4. Save current grid to file");
    println!("5. Load grid from file");
    println!("6. Display help");
    println!("7. Exit");

    let mut tapcode = Tapcode::new("abcdefghiklmnopqrstuvwxyz", '.').expect("Failed to initialize Tapcode cipher.");

    loop {
        print!("\nEnter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => {
                println!("Note: Use '|' to separate words in Tapcode.");
                print!("Enter message to encode: ");
                io::stdout().flush().unwrap();

                let mut message = String::new();
                io::stdin().read_line(&mut message).unwrap();

                match tapcode.encode(&message.trim()) {
                    Ok(encoded) => println!("Encoded Tapcode: {}", encoded),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "2" => {
                println!("Note: Use '|' to separate words in Tapcode (e.g., ... .... | .... ....).");
                print!("Enter Tapcode to decode: ");
                io::stdout().flush().unwrap();

                let mut code = String::new();
                io::stdin().read_line(&mut code).unwrap();

                match tapcode.decode(&code.trim()) {
                    Ok(decoded) => println!("Decoded Message: {}", decoded),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "3" => {
                print!("Enter a new 5x5 alphabet grid (25 characters): ");
                io::stdout().flush().unwrap();

                let mut new_alphabet = String::new();
                io::stdin().read_line(&mut new_alphabet).unwrap();

                match Tapcode::new(new_alphabet.trim(), tapcode.tap_symbol) {
                    Ok(new_tapcode) => {
                        tapcode = new_tapcode;
                        println!("{}", "Custom grid set successfully!".green());
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            "4" => save_grid_to_file(&tapcode),
            "5" => match load_grid_from_file() {
                Ok(grid) => match Tapcode::new(&grid, tapcode.tap_symbol) {
                    Ok(new_tapcode) => {
                        tapcode = new_tapcode;
                        println!("{}", "Grid loaded successfully!".green());
                    }
                    Err(e) => println!("Error: {}", e),
                },
                Err(e) => println!("Error: {}", e),
            },
            "6" => display_help(),
            "7" => {
                println!("{}", "Goodbye!".cyan());
                break;
            }
            _ => println!("Invalid choice. Please enter a number between 1 and 7."),
        }
    }
}
