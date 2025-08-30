//imports
use std::{io, thread, time};
use colored::*;
use rand::Rng;

// list of dokis
const DOKIS: [&str; 4] = ["Monika", "Sayori", "Yuri", "Natsuki"];

// it was at this point in the code skuffy realised he didn't want to write comments anymore
fn intro(){
        println!(
        "{}{}{}",
        "Random Doki Generator".magenta(),
        " - ",
        "Rust Edition".red()
    );

    println!(
        "{}{}{}",
        "Made by: ",
        "Skuffy".yellow(),
        " (https://github.com/skuffy808)"
    );
    println!("{}", "==============================================");
}

fn prompt_user() -> String {
    println!(
        "{}{}{}{}{}{}{}{}",
        "Would you like to generate a random ",
        "Doki".magenta(),
        "?",
        " (",
        "Y".green(),
        "/",
        "N".red(),
        ")"  
    );

    let mut response = String::new();

    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");

    response.trim().to_lowercase()
}

fn generate_doki() -> &'static str {
    let index = rand::thread_rng().gen_range(0..DOKIS.len());
    DOKIS[index]
}

fn main() {
    intro();
    loop {
        let response = prompt_user();

        if response == "y" {
            println!("{}", "Generating a random Doki...".green());
            println!(
                "{}{}{}",
                "You got: ",
                 generate_doki().magenta(),
                "!"
                );
            println!("{}", "closing in 5 seconds... (definitly not to lazy to add another loop)");
            thread::sleep(time::Duration::from_secs(5)); // ok i lied i am going to comment this one, this waits 5 seconds before quitting so that you can read the message
            break;
        }
        else if response == "n" {
            println!("{}", "Since you are not a Doki fan I am going to quit the program. OK?".red());
            thread::sleep(time::Duration::from_secs(2)); // and so does this one
            break;
        } else {
            println!("{}", "Invalid Input (\"please answer either \"y\" or \"n\")".red());
        }
    }
}
