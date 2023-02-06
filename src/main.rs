// Modules
mod enigma;
mod plug;
mod plugboard;
mod reflector;
mod rotor;

// Imports
use crate::{
    enigma::EnigmaMachine,
    plugboard::PlugBoard,
    reflector::{Reflector, ReflectorType},
    rotor::{Rotor, RotorType},
};

fn main() {
    // Create an Enigma machine
    let mut enigma_machine = EnigmaMachine::new(
        PlugBoard::new(),
        Reflector::new(ReflectorType::B),
        [
            Rotor::new(RotorType::I, 10),
            Rotor::new(RotorType::V, 20),
            Rotor::new(RotorType::IV, 0),
        ],
    );

    // Add some plugs
    enigma_machine.add_plugs("bu xh qz nf oa mt cg jv ly");

    // Encrypt and decrypt a string
    let start = "Hello, World!";
    let result = enigma_machine.send_string(start);
    let orginal = enigma_machine.send_string(&result);

    // Print the results
    println!("{} -> {} -> {}", start, result, orginal);
}
