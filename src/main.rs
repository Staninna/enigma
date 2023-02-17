use crate::enigma::EnigmaMachineState;

// Modules
mod enigma;
mod plug;
mod plugboard;
mod reflector;
mod rotor;

// Constants
const MESSAGE: &str = "helloworld";
const UPDATE_INTERVAL: u64 = 50000;

fn main() {
    // Encrypt the message
    let encrypted = EnigmaMachineState::random()
        .new_machine()
        .send_string(MESSAGE);

    // Loop until the message is cracked
    let mut machine = EnigmaMachineState::default();
    let mut attempts = 0;
    let mut cracked;
    let start = std::time::Instant::now();
    loop {
        // Send the message through the machine
        cracked = machine.new_machine().send_string(&encrypted);

        // Increment the attempts
        attempts += 1;

        // Update screen
        if attempts % UPDATE_INTERVAL == 0 {
            // Print progress
            clear();
            println!("Attempts: {}", attempts);
            println!("Time: {}s            ", start.elapsed().as_secs());
            println!(
                "Speed: {} attempts/s      ",
                attempts / start.elapsed().as_secs().max(1)
            );
            println!("{}", machine);
        }

        // Check if the message is cracked
        if cracked == MESSAGE {
            break;
        }

        // Update the machine
        machine = machine.next();
    }

    // Print the message
    clear();
    println!("Attempts: {}", attempts);
    println!("Message: {}", cracked);
    println!("Time: {}s          ", start.elapsed().as_secs());
    println!(
        "Speed: {} attempts/s      ",
        attempts / start.elapsed().as_secs().max(1)
    );
    println!("Config: {}", machine);
}

fn clear() {
    print!("{}[2J", 27 as char);
    print!("{}[1;1H", 27 as char);
}
