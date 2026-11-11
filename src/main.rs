use std::io;
use std::io::Write;
use std::fs;
use clap::{Parser, Subcommand};
use shlex;

#[derive(Parser)]
struct CliArgs {
    file_path: String,
}

#[derive(Parser)]
#[command(multicall = true)]
struct ReplArgs {
    #[command(subcommand)]
    command: ReplCommand,
}

#[derive(Debug, Subcommand)]
pub enum ReplCommand {
    /* /// Runs the ????
    Run, */
    /// Restricts the length of the words list
    Length {
        /// Length of the words
        length: usize,
    },
    /// Add a Word
    Aw {
        /// What is the word
        word: String,
        /// Out of asd, what is the Wordle encoding?
        info: String
    },
    /// Add a letter
    Al {
        /// What is the letter
        letter: String,
        /// Out of (a)sd, what is the Wordle encoding for this letter?
        info: String
    },
    /// Reset the words list to what is stored in the words list file
    Reset,
    /// Runs reset, then length <number>
    Rsl {
        /// Length of the words
        length: usize,
    },
    /// Prints the remaining words
    List,
    /// Writes the words to a file
    Write,
    /// Exits the program
    Exit,
}

fn main() {
    //println!("Hello, world!");
    
    // Collect command line inputs
    let CliArgs { file_path } = CliArgs::parse();

    let mut file_buf = String::new();
    let mut words = read_words(&file_path, &mut file_buf);
    
    println!("[INFO] aw only works when the ALL words are the correct length. Use length <number> to only use the words of <number> length");
    println!("       Otherwise, al still works, except you cant add a letters");
    println!("Use help for help");
    println!("Press ^C (CTRL + C) to exit");
    
    let mut still_running = true;
    while still_running {
        
        let Some(args) = shlex::split(&command_input(&words.len())) else {
            println!("AAAAAA");
            continue;
        };
        let args = match ReplArgs::try_parse_from(args) {
            Ok(args) => args,
            Err(e) => {println!("{}", e); continue;},
        };
        // println!("{:?}", args.command);
        
        match args.command {
            ReplCommand::Length { length } => {
                words = restrict_num_letters(length, words)
            },
            ReplCommand::List => {
                for word in &words {
                    println!("{word}");
                }
            },
            ReplCommand::Reset => {
                words = read_words(&file_path, &mut file_buf);
            },
            ReplCommand::Rsl { length } => {
                words = restrict_num_letters(length, read_words(&file_path, &mut file_buf));
                
            },
            ReplCommand::Aw { word, info } => {
                if word.len() != info.len() {
                    println!("aw requires the word and the asd info to be the same length!");
                } else {
                    words = a_w(word, info, words);
                }
            },
            ReplCommand::Al { letter, info } => {
                if letter.len() != info.len() {
                    println!("al requires the letter/s and the asd info to be the same length!");
                } else {
                    // We have just checked whether the letter String and the info String are the same length
                    words = a_l(letter, info, words);
                }
            }
            ReplCommand::Write => {
                fs::write("./possibleWords.txt", {
                let mut to_write: String = String::new();
                for word in &words {
                    to_write.push_str(word);
                    to_write.push_str("\n");
                }
                to_write
                }).expect("Could not write")
            }
            ReplCommand::Exit => {
                still_running = false;
            }
        }
    }
    println!("Bye!");
}

fn read_words<'a>(file_path: &String, file_buf: &'a mut String) -> Vec<&'a str> {
    *file_buf = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    *file_buf = file_buf.replace("\r", "");
    let output: Vec<&str> = file_buf 
        // .replace("\r", "")
        .split("\n")
        .collect();
    // // This is for windows (CRLF system)
    // let mut output: Vec<&str> = file_buf.split("\r\n").collect();
    // if output.len() == 1 {
    //     // For unix like systems (LF)
    //     output = file_buf.split("\n").collect();
    // }
    return output;
}

fn command_input(words_len: &usize) -> String {
    write!(io::stdout(), "({words_len}) >> ").expect("failed to print stdout");
    io::stdout().flush().expect("failed to flush stdout");
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    input.trim().to_owned()
}

fn restrict_num_letters(number_of_letters: usize, words: Vec<&str>) -> Vec<&str> {
    words.into_iter()
        .filter(|word: &&str| 
            {word.len() == number_of_letters
        })
        .collect()
}

fn a_l<'a>(letter: String, sd: String, words: Vec<&'a str>) -> Vec<&'a str> {
    let letter_vec: Vec<char> = letter.chars().collect();
    let sd_vec: Vec<char> = sd.chars().collect();
    
    let mut s_vec: Vec<char> = Vec::new();
    let mut d_vec: Vec<char> = Vec::new();
    
    for i in 0..letter_vec.len() {
        if sd_vec[i] == 's' {
            s_vec.push(letter_vec[i]);
        } else {
            d_vec.push(letter_vec[i]);
        }
    }
    
    words.into_iter().filter(|word|{
        check_word_a_l(word.to_string(), &s_vec, &d_vec)
    }).collect()
}

fn check_word_a_l(word: String, s_vec: &Vec<char>, d_vec: &Vec<char>) -> bool {
    for letter in s_vec {
        if !word.contains(*letter) {
            return false;
        }
    };
    for letter in d_vec {
        if word.contains(*letter) {
            return false;
        }
    }
    true
}

fn a_w<'a>(word: String, asd: String, words: Vec<&'a str>) -> Vec<&'a str> {
    let asd_vec: Vec<char> = asd.chars().collect();
    let word_vec: Vec<char> = word.chars().collect();
    
    // Setup the asd Vecs
    let mut a_indexes: Vec<Option<char>> = Vec::new();
    let mut s_letters: Vec<char> = Vec::new();
    let mut s_indexes: Vec<Option<char>> = Vec::new();
    let mut d_letters: Vec<char> = Vec::new();
    for i in 0..asd_vec.len() {
        // println!("{}", i);
        if asd_vec[i] == 'a' {
            a_indexes.push(Some(word_vec[i]));
        } else {
            a_indexes.push(None);
        }
        if asd_vec[i] == 's' {
            s_letters.push(word_vec[i]);
            s_indexes.push(Some(word_vec[i]));
        } else {
            s_indexes.push(None);
        }
        if asd_vec[i] == 'd' {
            d_letters.push(word_vec[i]);
        }
    };
    
    words.into_iter().filter(|word| {
            check_word_a_w(word.to_string(), &a_indexes, &s_letters, &s_indexes, &d_letters)
        }
    ).collect()
}

fn check_word_a_w(word: String, a_indexes: &Vec<Option<char>>, s_letters: &Vec<char>, s_indexes: &Vec<Option<char>>, d_letters: &Vec<char>) -> bool {
    let word_vec: Vec<char> = word.chars().collect();
    let mut s_letters_mut = s_letters.clone();
    
    for i in 0..word_vec.len() {
        let letter = word_vec[i];
        
        if a_indexes[i] != None {
            if Some(letter) != a_indexes[i] {
                return false;
            } else {
                // This letter is correct
                continue;
            }
        }
        
        if s_indexes[i] != None {
            if Some(letter) == s_indexes[i] {
                return false;
            }
        }
        
        let s_letter_idx = s_letters_mut.iter().position(|l| l == &letter);
        if s_letter_idx != None {
            s_letters_mut.remove(s_letter_idx.unwrap());
        }
        
        if d_letters.contains(&letter) {
            return false;
        }
    };
    
    // If there are still required letters, return false
    if s_letters_mut.len() != 0 {
        return false;
    }
    
    // If nothing has returned false, return true
    true
}
