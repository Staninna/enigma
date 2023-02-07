// Imports
use crate::{plugboard::PlugBoard, reflector::Reflector, rotor::Rotor};

#[derive(Clone)]
pub struct EnigmaMachineTemplate {
    pub plugboard: PlugBoard,
    pub reflector: Reflector,
    pub rotors: [Rotor; 3],
}

impl EnigmaMachineTemplate {
    pub fn new(plugboard: PlugBoard, reflector: Reflector, rotors: [Rotor; 3]) -> Self {
        Self {
            plugboard,
            reflector,
            rotors,
        }
    }

    pub fn new_machine(&self) -> EnigmaMachine {
        EnigmaMachine::new(self.clone())
    }
}

// An Enigma machine
pub struct EnigmaMachine {
    plugboard: PlugBoard,
    reflector: Reflector,
    rotors: [Rotor; 3],
}

impl EnigmaMachine {
    // Create a new Enigma machine
    fn new(config: EnigmaMachineTemplate) -> Self {
        Self {
            plugboard: config.plugboard,
            reflector: config.reflector,
            rotors: config.rotors,
        }
    }

    // Encrypt and decrypt a string
    pub fn send_string(&mut self, data: &str) -> String {
        let mut normalized_data = data.to_string().to_lowercase();
        normalized_data.retain(|c| c.is_alphabetic());

        let mut result = String::new();
        for c in normalized_data.chars() {
            result.push(self.send(c));
        }
        result
    }

    // Encrypt and decrypt a character
    fn send(&mut self, data: char) -> char {
        let mut result = data;
        result = self.plugboard.move_data(result);

        result = self.rotors[0].forward(result);
        result = self.rotors[1].forward(result);
        result = self.rotors[2].forward(result);

        result = self.reflector.reflect(result);

        result = self.rotors[2].backward(result);
        result = self.rotors[1].backward(result);
        result = self.rotors[0].backward(result);

        result = self.plugboard.move_data(result);

        self.rotate();

        result
    }

    // Rotate the rotors
    fn rotate(&mut self) {
        self.rotors[0].rotate();

        if self.rotors[0].is_notch() {
            self.rotors[1].rotate();
        }

        if self.rotors[1].is_notch() {
            self.rotors[2].rotate();
        }
    }
}
