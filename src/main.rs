// Rust implementation
// of a Game Theory's game "Prisoner's Dilemma"
// Added an additional choice 'StaySilent'
// Skips your turn at no consequences
// If opponent also stays silent.

/* Paradigm Delta */

// Returns an iterator of the CLA passed to the program
use std::io;
mod game;
use game::*;

// Server & Client files
mod service;


// Take player 1 & player 2's usernames in a single line separated by a space
fn get_agent_names(whole_string : &String) -> (String, String) {

    let str_as_bytes: &[u8] = whole_string.as_bytes();
    let end_of_str: usize = whole_string.len();

    let mut first_str: usize = 0;

    for (i, &item) in str_as_bytes.iter().enumerate() {
        if item == b' ' {
            first_str =  i;
            break;
        }
    }

    // Returns a tuple containing the two split name(s)
    (whole_string[0..first_str].to_string() , whole_string[first_str+1..end_of_str].to_string() )
}


fn main() {
    
    println!("Welcome to Agent's Dilemma!");

    println!("\nInput usernames for Agent 1 & Agent 2: ");


    // Agent names are to be stored here
    let mut agent_names = String::new();

    // Obtain from CLA
    io::stdin().read_line(&mut agent_names)
    .expect("Failed to read line");

    // Create two separate names after being split
    let ( agent1 , agent2 ) = get_agent_names(&agent_names);

    println!("Agent 1: {agent1}");
    println!("Agent 2: {agent2}");

    // Create the player's struct
    let agent1: Agent = build_agent(&agent1);
    let agent2: Agent = build_agent(&agent2);

    
    println!("How-To-Play\nRules:\n\t1) Cooperate\n\t2) Defect\n\t3) Stay Silent\n
     >Input the Corresponding number...\nStarting game...");


    // Core Game logic: game.rs
    play_game(agent1, agent2);

}
