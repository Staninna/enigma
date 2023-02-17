// Imports
use crate::{
    plugboard::PlugBoard,
    reflector::{Reflector, ReflectorType},
    rotor::{Rotor, RotorType},
};

// A state of the Enigma machine
#[derive(Clone)]
pub struct EnigmaMachineState {
    pub plugboard: PlugBoard,
    pub reflector: Reflector,
    rotors: [Rotor; 3],
}

impl EnigmaMachineState {
    // Create a new machine state
    #[allow(dead_code)]
    pub fn new(plugboard: PlugBoard, reflector: Reflector, rotors: [Rotor; 3]) -> Self {
        Self {
            plugboard,
            reflector,
            rotors,
        }
    }

    // Default state
    pub fn default() -> Self {
        Self {
            plugboard: PlugBoard::new(""),
            reflector: Reflector::new(ReflectorType::B),
            rotors: [
                Rotor::new(RotorType::I, 'a'),
                Rotor::new(RotorType::I, 'a'),
                Rotor::new(RotorType::I, 'a'),
            ],
        }
    }

    // Create a new machine based on the state
    pub fn new_machine(&self) -> EnigmaMachine {
        EnigmaMachine::new(
            self.plugboard.clone(),
            self.reflector.clone(),
            self.rotors.clone(),
        )
    }

    pub fn max() -> Self {
        Self::new(
            PlugBoard::new(""), // TODO: Can't enumerate the plugboard yet
            Reflector::new(ReflectorType::C),
            [
                Rotor::new(RotorType::V, 'z'),
                Rotor::new(RotorType::V, 'z'),
                Rotor::new(RotorType::V, 'z'),
            ],
        )
    }

    // Create a random machine state
    #[allow(dead_code)]
    pub fn random() -> Self {
        Self {
            plugboard: PlugBoard::new(""), // TODO: Can't enumerate the plugboard yet
            reflector: Reflector::random(),
            rotors: [Rotor::random(), Rotor::random(), Rotor::random()],
        }
    }

    // Increment the state
    #[allow(dead_code)]
    pub fn next(&mut self) -> Self {
        // Convert the state char array
        let mut chars = self.to_chars();

        // Increment the state
        increment_state(&mut chars);

        // Convert the state char array
        self.from_chars(chars)
    }

    // TODO: Plugboard
    // Convert the state to a char array
    pub fn to_chars(&self) -> [char; 7] {
        let mut result = [0 as char; 7];
        result[0] = self.rotors[0].position;
        result[1] = self.rotors[1].position;
        result[2] = self.rotors[2].position;
        result[3] = char::from(&self.rotors[0].rotor_type);
        result[4] = char::from(&self.rotors[1].rotor_type);
        result[5] = char::from(&self.rotors[2].rotor_type);
        result[6] = char::from(&self.reflector.reflector_type);

        result
    }

    // TODO: Plugboard
    // Convert a char array to a state
    pub fn from_chars(&mut self, chars: [char; 7]) -> Self {
        Self::new(
            PlugBoard::new(""), // TODO: Can't enumerate the plugboard yet
            Reflector::new(ReflectorType::from(chars[6])),
            [
                Rotor::new(RotorType::from(chars[3]), chars[0]),
                Rotor::new(RotorType::from(chars[4]), chars[1]),
                Rotor::new(RotorType::from(chars[5]), chars[2]),
            ],
        )
    }
}

impl std::fmt::Display for EnigmaMachineState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}",
            self.rotors[0].position,
            self.rotors[1].position,
            self.rotors[2].position,
            char::from(&self.rotors[0].rotor_type),
            char::from(&self.rotors[1].rotor_type),
            char::from(&self.rotors[2].rotor_type),
            char::from(&self.reflector.reflector_type),
        )
    }
}

// TODO: Plugboard
// Increment the state
fn increment_state(chars: &mut [char; 7]) {
    // Increment the rotors
    let mut i = 0;
    let mut last_wrap = false;
    while i < 3 && chars[i] == 'z' {
        chars[i] = 'a';

        // If the last rotor is wrapped, we need to swap the rotor type
        last_wrap = (i + 1) == 3;

        i += 1;
    }
    if i < 3 {
        let new_rotor = char::from_u32(chars[i] as u32 + 1).unwrap_or('a');
        chars[i] = new_rotor;
    }

    // Stop if the last rotor is not wrapped
    if !last_wrap {
        return;
    }

    // Swap the roter types
    i = 3;
    last_wrap = false;
    while i < 6 && chars[i] == '5' {
        chars[i] = '1';

        // If the last rotor is wrapped, we need to swap the reflector
        last_wrap = (i + 1) == 6;

        i += 1;
    }
    if i < 6 {
        let new_rotor = char::from_u32(chars[i] as u32 + 1).unwrap_or('1');
        chars[i] = new_rotor;
    }

    // Stop if the last rotor type is not wrapped
    if !last_wrap {
        return;
    }

    // Swap the reflector
    if chars[6] == 'c' {
        chars[6] = 'b';
    } else {
        chars[6] = 'c';
    }
}

// An Enigma machine
pub struct EnigmaMachine {
    pub plugboard: PlugBoard,
    pub reflector: Reflector,
    pub rotors: [Rotor; 3],
}

impl EnigmaMachine {
    // Create a new Enigma machine
    fn new(plugboard: PlugBoard, reflector: Reflector, rotors: [Rotor; 3]) -> Self {
        Self {
            plugboard,
            reflector,
            rotors,
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
