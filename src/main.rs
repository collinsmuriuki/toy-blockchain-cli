extern crate cheza_blockchain;

use std::io;
use std::io::Write;
use std::process;

use cheza_blockchain::Chain;

fn main() {
    let mut miner_address = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    println!("Input a miner address: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut miner_address).unwrap();
    println!("Difficulty: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut difficulty).unwrap();
    let diff = difficulty
        .trim()
        .parse::<u32>()
        .expect("We need an integer");

    let mut chain = Chain::new(miner_address.trim().to_string(), diff);

    loop {
        println!("== Menu ==");
        println!("1. New Transaction");
        println!("2. Mine block");
        println!("3. Change difficulty");
        println!("4. Change reward");
        println!("0. Exit");
        println!("Enter your choice");
        io::stdout().flush().unwrap();
        choice.clear();
        io::stdin().read_line(&mut choice).unwrap();
        println!("");

        match choice.trim().parse().unwrap() {
            0 => {
                println!("Bye");
                process::exit(0);
            }
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                println!("Enter sender address: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut sender).unwrap();
                println!("Enter receiver address: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut receiver).unwrap();
                println!("Enter amount: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut amount).unwrap();

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap(),
                );

                match res {
                    true => println!("Transaction added"),
                    false => println!("Transaction failed"),
                }
            }
            2 => {
                println!("Generating block");
                let res = chain.generate_new_block();

                match res {
                    true => println!("Block generatied successfuly"),
                    false => println!("Block generation failed"),
                }
            }
            3 => {
                let mut new_diff = String::new();
                println!("Enter new difficulty: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut new_diff).unwrap();

                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());

                match res {
                    true => println!("Updated difficulty to {}", new_diff),
                    false => println!("Failed to update difficulty"),
                }
            }
            4 => {
                let mut new_reward = String::new();
                println!("Enter new reward: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut new_reward).unwrap();

                let res = chain.update_reward(new_reward.trim().parse().unwrap());

                match res {
                    true => println!("Updated reward to {}", new_reward),
                    false => println!("Failed to update reward"),
                }
            }
            _ => {
                println!("\nInvalid option please retry\n");
                continue;
            }
        }
    }
}
