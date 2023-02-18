use crate::enigma::EnigmaMachineState;
use std::thread;

// Modules
mod enigma;
mod plug;
mod plugboard;
mod reflector;
mod rotor;

// Constants
const MESSAGE: &str = "helloworld";
const TOTAL_STATES: u64 = 2 * 26 * 26 * 26 * 5 * 5 * 5;
const TREADS: u64 = 8;
const TREAD_NAMES: [&str; 8] = [
    "Wicher", "Jasper", "Vicky", "Stan", "Bart", "Ruben", "Cheryl", "Benjamin",
];

fn main() {
    // Encrypt the message using a random enigma machine
    let encrypted = EnigmaMachineState::random()
        .new_machine()
        .send_string(MESSAGE.to_string());

    // Create a state for each thread
    let mut states = Vec::new();
    for i in 0..TREADS {
        let start = i * TOTAL_STATES / TREADS;
        let state = EnigmaMachineState::from_index(start);
        states.push(state);
    }

    // Create a thread for each state
    let mut threads = Vec::new();
    for state in states {
        // Clone the needed variables
        let encrypted = encrypted.clone();
        let mut state = state;

        // Create the thread
        let name = TREAD_NAMES[threads.len()];
        let thread = thread::Builder::new()
            .name(name.to_string())
            .spawn(move || {
                // Initialize the variables
                let mut decrypted;
                let mut attempts = 0;
                let start_time = std::time::Instant::now();
                let end_time;

                // Loop until the message is cracked
                loop {
                    decrypted = state.new_machine().send_string(encrypted.clone());
                    attempts += 1;
                    if decrypted == MESSAGE {
                        end_time = std::time::Instant::now();
                        break;
                    } else {
                        state = state.next();
                    }
                }

                // Print the results
                println!(
                    "Sultion found on thread: {}",
                    std::thread::current().name().unwrap()
                );
                println!("Decrypted: {}", decrypted);
                println!("Configuration: {}", state);
                println!("Time: {:?}", end_time - start_time);
                println!("Attempts: {}", attempts);
                println!(
                    "Attempts per second: {}",
                    attempts as f64 / (end_time - start_time).as_secs_f64()
                );

                // Exit the program to prevent the other threads from running
                std::process::exit(0);
            })
            .unwrap();

        threads.push(thread);
    }

    // Wait for all threads to finish
    #[allow(unused_must_use)]
    for thread in threads {
        thread.join();
    }
}
