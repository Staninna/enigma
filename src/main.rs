// Modules
mod enigma;
mod plug;
mod plugboard;
mod reflector;
mod rotor;

// Imports
use crate::{
    enigma::EnigmaMachineTemplate as EnigmaMachine,
    plugboard::PlugBoard,
    reflector::{Reflector, ReflectorType},
    rotor::{Rotor, RotorType},
};

fn main() {
    // Create an Enigma machine config
    let machine = EnigmaMachine::new(
        PlugBoard::new("AB CD EF GH IJ KL MN OP QR ST UV WX YZ"),
        Reflector::new(ReflectorType::B),
        [
            Rotor::new(RotorType::I, 10),
            Rotor::new(RotorType::V, 20),
            Rotor::new(RotorType::IV, 0),
        ],
    );

    // Create an Enigma machine
    let mut enigma_machine = machine.new_machine();

    // Encrypt and a string
    let start = "Hello, World!";
    let result = enigma_machine.send_string(start);

    // Create a new Enigma machine
    let mut enigma_machine = machine.new_machine();

    // Decrypt the result
    let orginal = enigma_machine.send_string(&result);

    // Print the results
    println!("{} -> {} -> {}", start, result, orginal);
}
