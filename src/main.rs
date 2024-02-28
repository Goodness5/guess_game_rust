extern crate cfonts;
use std::{cmp::Ordering, io::{self, Write}};

use cfonts::{say, Options, Align, Colors, Fonts}; // Import modules from cfonts crate
use rand::Rng;

fn main() {
    // Generate a random number for the CPU's guess
    let cpu_guess: u64 = rand::thread_rng().gen_range(1..20);

    // Define options for the prompt
    let prompt_options = Options {
        text: String::from("HELLO MOTHERFUCKER MAKE A FUCKING GUESS BETWEEN 1 AND 20 "),
        colors: vec![Colors::Blue, Colors::White],
        line_height: 1,
        gradient: vec!["#ff4400".to_string(), "#0050ff".to_string()],
        align: Align::Center,
        font: Fonts::FontChrome,
        spaceless: true,
        letter_spacing: 1,
        ..Options::default()
    };
    let error = Options {
        text: String::from("wtf did you just input TRY AGAIN!"),
        colors: vec![Colors::Blue, Colors::White],
        line_height: 1,
        gradient: vec!["#ff4400".to_string(), "#0050ff".to_string()],
        align: Align::Center,
        font: Fonts::FontChrome,
        spaceless: true,
        letter_spacing: 1,
        ..Options::default()
    };
    say(prompt_options);
    loop {
        
        let mut guess = String::new();
        say(Options {
            text: String::from("Your guess: "),
            colors: vec![Colors::Blue, Colors::White],
            align: Align::Center,
            font: Fonts::FontTiny,
            ..Options::default()
        });

        // Read user input
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                // Display error message and continue loop
                say(error.clone());
                continue;
            }
        };
        match guess.cmp(&cpu_guess) {
            Ordering::Equal => {
                // Display the user's guess
                say(Options {
                    text: String::from("You guessed Right! Congratulations motherfucker"),
                    colors: vec![Colors::Green, Colors::White],
                    align: Align::Center,
                    font: Fonts::FontTiny,
                    ..Options::default()
                });
                break;
            },
            Ordering::Greater => {
                // Display a message for a wrong guess and the correct number
                say(Options {
                    text: String::from("You guessed Greater!"),
                    colors: vec![Colors::RedBright, Colors::White],
                    align: Align::Center,
                    font: Fonts::FontTiny,
                    ..Options::default()
                });

            },
            Ordering::Less => {
                // Display a message for a wrong guess and the correct number
                say(Options {
                    text: String::from("You guessed Lesser!"),
                    colors: vec![Colors::RedBright, Colors::White],
                    align: Align::Center,
                    font: Fonts::FontSimple,
                    ..Options::default()
                });
            },
        }
    }
    }


   
