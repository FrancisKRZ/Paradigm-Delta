/*
    Game Logic

*/


// Used in silent_development()
use rand::Rng;
use std::io;
use std::io::Write; // flush() for print


// Utilize threads & networks
// threads wait for either player's 
// input on the network
use std::thread;

// Player struct, counting the ENUM choices
// Silent - Testify 
pub struct Agent {

    name: String,
    points: u8,
}

// Implementation of the player's stats
impl Agent {
  
    fn get_name(&mut self) -> String {
        let ret_name: String = self.name.to_string();
        return ret_name;
    }

    fn get_points(&mut self) -> u8 {
        return self.points;
    }

    fn increase_points(&mut self, pts: u8) {

        self.points += pts;
    }

}

// External fn initiating the Agent instance
pub fn build_agent(name: &String) -> Agent {

    Agent {
        name: String::from(name),
        points: 0,
    }
}


// Enum of choices, Cooperate or Defect
enum AgentChoices {
    Cooperate,
    Defect,
    StaySilent,
}

impl AgentChoices {
    fn from_u8(choice: u8) -> Option<Self> {
        match choice {
            1 => Some(AgentChoices::Cooperate),
            2 => Some(AgentChoices::Defect),
            3 => Some(AgentChoices::StaySilent),
            _ => None,          
        }
    }
}


fn silent_development(agent : &mut Agent) {

    // 75% for 2 to 5 points 25% for 0 points
    let rng_num : u8 = rand::thread_rng().gen_range(2..6);

    // Four is zero!
    let rng_pts: u8 = if rng_num == 4 {0} else {rng_num};

    agent.increase_points(rng_pts);
} 

// Decides who gets the appropriate points
fn game_round(agent1_choice : AgentChoices, mut agent1 : &mut Agent,
              agent2_choice : AgentChoices, mut agent2 : &mut Agent ) {

    let agent1_usr: String = agent1.get_name();
    let agent2_usr: String = agent2.get_name();

    match agent1_choice {

        AgentChoices::Cooperate => {

            match agent2_choice {
                // Neither develops technology
                AgentChoices::Cooperate => {agent1.increase_points(1);
                                           agent2.increase_points(1);
                                           println!("\n\
                                           {agent1_usr} and {agent2_usr} Cooperated.\nBoth won 1 points!\n");
                                        }
                // Agent 1 does not - Agent 2 does
                AgentChoices::Defect => {agent1.increase_points(0);
                                         agent2.increase_points(3);
                                         println!("\n\
                                         {agent1_usr} Cooperated, won 0 points :( - \
                                         {agent2_usr} Defected, won 3 points!\n");
                                        }
                
                AgentChoices::StaySilent => {agent1.increase_points(
                                            rand::thread_rng().gen_range(2..4));
                                            silent_development(&mut agent2);
                                            println!("\n{agent1_usr} Cooperated - {agent2_usr} Stayed Silent...\n");
                                        }
            }
        } // eof C - a2ch

        AgentChoices::Defect => {
        
            println!("Agent 2 has chosen: ");
            match agent2_choice {
                // Agent 1 develops technology
                AgentChoices::Cooperate => {agent1.increase_points(3);
                                           agent2.increase_points(0);
                                           println!("\n\
                                           {agent1_usr} Defected! Won 3 Points - \
                                           {agent2_usr} Cooperated, won 0 points :(\n");
                                        }
                // Agent 1 does - Agent 2 does not
                AgentChoices::Defect => {agent1.increase_points(2);
                                         agent2.increase_points(2);
                                         println!("\nBoth have defected! - 2 points each\n");
                                        }
                
                AgentChoices::StaySilent => {silent_development(&mut agent2);
                                             agent1.increase_points(rand::thread_rng().gen_range(2..4));
                                             println!("\n\
                                             {agent1_usr} Defected - \
                                             {agent2_usr} Stayed Silent...\n");
                                            }
            }
        } // eof D - a2ch

        AgentChoices::StaySilent => {

            silent_development(&mut agent1);

            match agent2_choice {
                // Neither develops technology
                AgentChoices::Cooperate => {agent2.increase_points(rand::thread_rng().gen_range(2..4));
                                            println!("\n{agent1_usr} stayed silent... {agent2_usr} Cooperated\n")
                                            }
                // Agent 1 does not - Agent 2 does
                AgentChoices::Defect => {agent2.increase_points(rand::thread_rng().gen_range(2..4));
                                        println!("\n{agent1_usr} stayed silent... {agent2_usr} Defected\n");
                                        }
                
                AgentChoices::StaySilent => {silent_development(&mut agent2);
                                            println!("\nBoth stayed silent...\n");
                                            }
            }
        } // eof S - a2ch

    } // eof match



}


fn read_agent_input(agent : &mut Agent) -> u8 {


    let agent_name: String = agent.get_name();

    io::stdout().flush().unwrap();
    print!("{agent_name}'s turn: ");

    
    // mut ?
    let agent_input_u8: u8;

    loop {

        // 1 - Cooperate
        // 2 - Defect
        // 3 - Stay Silent
        let mut agent_input: String = String::new();
        io::stdin()
        .read_line(&mut agent_input)
        .expect("Failed to read from stdin");
        

        let agent_input_convert: u8 = match agent_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        if (1..=3).contains(&agent_input_convert){
            agent_input_u8 = agent_input_convert;
            break;
        } else {
            println!("Invalid input '{}', must be 0..3", agent_input_convert);
        };

    }

    // println!("Returning with: {agent_input_u8}");
    agent_input_u8

}


pub fn play_game(mut agent1 : Agent, mut agent2 : Agent) {

    // Main game loop

    let mut game_counter: u8 = 1;

    loop {

        println!("Game round {}... start!", game_counter);

        // Certified Rust moment
        let agent1_choice: AgentChoices = 
        AgentChoices::from_u8(read_agent_input(&mut agent1)).unwrap();

        let agent2_choice: AgentChoices = 
        AgentChoices::from_u8(read_agent_input(&mut agent2)).unwrap();

        // Computes round winner
        game_round(agent1_choice, &mut agent1, agent2_choice, &mut agent2);

        game_counter += 1;

        // concludes
        if game_counter == 10 {break;}

    }

    println!("Game finished... \nWinner: ");
    
    let ap1: u8 = agent1.get_points();
    let ap2: u8 = agent2.get_points();
    let au1: String = agent1.get_name();
    let au2: String = agent2.get_name();

    if ap1 > ap2 {
        println!("{au1} defeated {au2}!\n{ap1} > {ap2}");
    } else if ap1 < ap2 {
        println!("{au2} defeated {au1}!\n{ap2} > {ap1}");
    } else {
        println!("It's a tie!\n{ap1} == {ap2}\nRejoice!");
    }


}


