// All possible rotor types
#[allow(dead_code)]
pub enum RotorType {
    I,
    II,
    III,
    IV,
    V,
}

// A rotor
#[derive(Clone)]
pub struct Rotor {
    notch: char,
    pub position: char,
    wiring: [char; 26],
}

impl Rotor {
    // Create a new rotor based on given rotor type and position
    pub fn new(rotor_type: RotorType, position: u8) -> Self {
        match rotor_type {
            RotorType::I => Self {
                notch: 'y',
                position: (position + 97) as u8 as char,
                wiring: [
                    'e', 'k', 'm', 'f', 'l', 'g', 'd', 'q', 'v', 'z', 'n', 't', 'o', 'w', 'y', 'h',
                    'x', 'u', 's', 'p', 'a', 'i', 'b', 'r', 'c', 'j',
                ],
            },
            RotorType::II => Self {
                notch: 'm',
                position: (position + 97) as u8 as char,
                wiring: [
                    'a', 'j', 'd', 'k', 's', 'i', 'r', 'u', 'x', 'b', 'l', 'h', 'w', 't', 'm', 'c',
                    'q', 'g', 'z', 'n', 'p', 'y', 'f', 'v', 'o', 'e',
                ],
            },
            RotorType::III => Self {
                notch: 'd',
                position: (position + 97) as u8 as char,
                wiring: [
                    'b', 'd', 'f', 'h', 'j', 'l', 'c', 'p', 'r', 't', 'x', 'v', 'z', 'n', 'y', 'e',
                    'i', 'w', 'g', 'a', 'k', 'm', 'u', 's', 'q', 'o',
                ],
            },
            RotorType::IV => Self {
                notch: 'r',
                position: (position + 97) as u8 as char,
                wiring: [
                    'e', 's', 'o', 'v', 'p', 'z', 'j', 'a', 'y', 'q', 'u', 'i', 'r', 'h', 'x', 'l',
                    'n', 'f', 't', 'g', 'k', 'd', 'c', 'm', 'w', 'b',
                ],
            },
            RotorType::V => Self {
                notch: 'h',
                position: (position + 97) as u8 as char,
                wiring: [
                    'v', 'z', 'b', 'r', 'g', 'i', 't', 'y', 'u', 'p', 's', 'd', 'n', 'h', 'l', 'x',
                    'a', 'w', 'm', 'j', 'q', 'o', 'f', 'e', 'c', 'k',
                ],
            },
        }
    }

    // Move data forward through the rotor
    pub fn forward(&self, data: char) -> char {
        let alphabet = "abcdefghijklmnopqrstuvwxyz";
        let offset = alphabet.find(self.position).unwrap();
        let alphabet = rotate_alphabet(alphabet, offset);
        let position = alphabet.find(data).unwrap();
        self.wiring[position]
    }

    // Move data backwards through the rotor
    pub fn backward(&self, data: char) -> char {
        let alphabet = "abcdefghijklmnopqrstuvwxyz";
        let offset = alphabet.find(self.position).unwrap();
        let alphabet = rotate_alphabet(alphabet, offset);
        let position = self.wiring.iter().position(|&x| x == data).unwrap();
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
