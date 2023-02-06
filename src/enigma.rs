// Imports
use crate::{plug::Plug, plugboard::PlugBoard, reflector::Reflector, rotor::Rotor};

// An Enigma machine
pub struct EnigmaMachine {
    plugboard: PlugBoard,
    reflector: Reflector,
    rotors: [Rotor; 3],
}

impl EnigmaMachine {
    // Create a new Enigma machine
    pub fn new(plugboard: PlugBoard, reflector: Reflector, rotors: Vec<Rotor>) -> Self {
        Self {
            plugboard,
            reflector,
            rotors: [rotors[0], rotors[1], rotors[2]],
        }
    }

    // Add plugs to the plugboard
    pub fn add_plugs(&mut self, plugs: &str) {
        // Normalize plugs
        let plugs = plugs.replace(" ", "").to_lowercase();

        // Check for invalid plugs
        for c in plugs.chars() {
            if !c.is_alphabetic() {
                panic!("Invalid plug: {}", c);
            }
        }

        // Check for even number of plugs
        if plugs.len() % 2 != 0 {
            panic!("Plugs must be in pairs");
        }

        // Check for duplicate plugs
        let mut plugs = plugs.chars().collect::<Vec<char>>();
        plugs.sort();
        for i in 0..plugs.len() - 1 {
            if plugs[i] == plugs[i + 1] {
                panic!("Duplicate plug: {}", plugs[i]);
            }
        }

        // Add plugs to plugboard
        for i in 0..plugs.len() / 2 {
            self.plugboard
                .add_plug(Plug::new(plugs[i * 2], plugs[i * 2 + 1]));
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
