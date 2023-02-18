use std::fmt::Display;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

// All possible rotor types
#[derive(Clone)]
pub enum RotorType {
    I,
    II,
    III,
    IV,
    V,
}

impl Display for RotorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RotorType::I => write!(f, "1"),
            RotorType::II => write!(f, "2"),
            RotorType::III => write!(f, "3"),
            RotorType::IV => write!(f, "4"),
            RotorType::V => write!(f, "5"),
        }
    }
}

impl From<&RotorType> for char {
    fn from(rotor_type: &RotorType) -> Self {
        match rotor_type {
            RotorType::I => '1',
            RotorType::II => '2',
            RotorType::III => '3',
            RotorType::IV => '4',
            RotorType::V => '5',
        }
    }
}

impl From<char> for RotorType {
    fn from(c: char) -> Self {
        match c {
            '1' => RotorType::I,
            '2' => RotorType::II,
            '3' => RotorType::III,
            '4' => RotorType::IV,
            '5' => RotorType::V,
            _ => panic!("Invalid rotor type"),
        }
    }
}

// A rotor
#[derive(Clone)]
pub struct Rotor {
    pub position: char,
    pub rotor_type: RotorType,
    wires: [char; 26],
    notch: char,
}

impl Rotor {
    // Create a new rotor based on given rotor type and position
    pub fn new(rotor_type: RotorType, position: char) -> Self {
        match rotor_type {
            RotorType::I => Self {
                rotor_type,
                position,
                // EKMFLGDQVZNTOWYHXUSPAIBRCJ
                wires: [
                    'e', 'k', 'm', 'f', 'l', 'g', 'd', 'q', 'v', 'z', 'n', 't', 'o', 'w', 'y', 'h',
                    'x', 'u', 's', 'p', 'a', 'i', 'b', 'r', 'c', 'j',
                ],
                notch: 'q',
            },
            RotorType::II => Self {
                rotor_type,
                position,
                // AJDKSIRUXBLHWTMCQGZNPYFVOE
                wires: [
                    'a', 'j', 'd', 'k', 's', 'i', 'r', 'u', 'x', 'b', 'l', 'h', 'w', 't', 'm', 'c',
                    'q', 'g', 'z', 'n', 'p', 'y', 'f', 'v', 'o', 'e',
                ],
                notch: 'e',
            },
            RotorType::III => Self {
                rotor_type,
                position,
                // BDFHJLCPRTXVZNYEIWGAKMUSQO
                wires: [
                    'b', 'd', 'f', 'h', 'j', 'l', 'c', 'p', 'r', 't', 'x', 'v', 'z', 'n', 'y', 'e',
                    'i', 'w', 'g', 'a', 'k', 'm', 'u', 's', 'q', 'o',
                ],
                notch: 'v',
            },
            RotorType::IV => Self {
                rotor_type,
                position,
                // ESOVPZJAYQUIRHXLNFTGKDCMWB
                wires: [
                    'e', 's', 'o', 'v', 'p', 'z', 'j', 'a', 'y', 'q', 'u', 'i', 'r', 'h', 'x', 'l',
                    'n', 'f', 't', 'g', 'k', 'd', 'c', 'm', 'w', 'b',
                ],
                notch: 'j',
            },
            RotorType::V => Self {
                rotor_type,
                position,
                // VZBRGITYUPSDNHLXAWMJQOFECK
                wires: [
                    'v', 'z', 'b', 'r', 'g', 'i', 't', 'y', 'u', 'p', 's', 'd', 'n', 'h', 'l', 'x',
                    'a', 'w', 'm', 'j', 'q', 'o', 'f', 'e', 'c', 'k',
                ],
                notch: 'z',
            },
        }
    }

    // Create a random rotor
    pub fn random() -> Self {
        let rotor_type = match rand::random::<u8>() % 5 {
            0 => RotorType::I,
            1 => RotorType::II,
            2 => RotorType::III,
            3 => RotorType::IV,
            4 => RotorType::V,
            _ => unreachable!(),
        };
        let position = (rand::random::<u8>() % 26 + 97) as char;

        // Create the rotor
        Self::new(rotor_type, position)
    }

    // Move data forward through the rotor
    pub fn forward(&self, data: char) -> char {
        let offset = ALPHABET.find(self.position).unwrap();
        let alphabet = rotate_alphabet(ALPHABET, offset);
        let position = alphabet.find(data).unwrap();
        self.wires[position]
    }

    // Move data backwards through the rotor
    pub fn backward(&self, data: char) -> char {
        let offset = ALPHABET.find(self.position).unwrap();
        let alphabet = rotate_alphabet(ALPHABET, offset);
        let position = self.wires.iter().position(|&x| x == data).unwrap();
        alphabet.chars().nth(position).unwrap()
    }

    // Rotate the rotor
    pub fn rotate(&mut self) {
        if self.position == 'z' {
            self.position = 'a';
        } else {
            self.position = (self.position as u8 + 1) as char;
        }
    }

    // Check if the rotor is at the notch
    pub fn is_notch(&self) -> bool {
        self.position == self.notch
    }
}

fn rotate_alphabet(alphabet: &str, offset: usize) -> String {
    alphabet
        .chars()
        .cycle()
        .skip(offset)
        .take(26)
        .collect::<String>()
}
