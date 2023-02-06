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
#[derive(Copy, Clone)]
pub struct Rotor {
    notch: char,
    position: char,
    wiring: [[char; 2]; 26],
}

impl Rotor {
    // Create a new rotor based on given rotor type and position
    pub fn new(rotor_type: RotorType, position: u8) -> Self {
        match rotor_type {
            RotorType::I => Self {
                notch: 'y',
                position: (position + 65) as u8 as char,
                wiring: [
                    ['a', 'e'],
                    ['b', 'k'],
                    ['c', 'm'],
                    ['d', 'f'],
                    ['e', 'l'],
                    ['f', 'g'],
                    ['g', 'd'],
                    ['h', 'q'],
                    ['i', 'v'],
                    ['j', 'z'],
                    ['k', 'n'],
                    ['l', 't'],
                    ['m', 'o'],
                    ['n', 'w'],
                    ['o', 'y'],
                    ['p', 'h'],
                    ['q', 'x'],
                    ['r', 'u'],
                    ['s', 's'],
                    ['t', 'p'],
                    ['u', 'a'],
                    ['v', 'i'],
                    ['w', 'b'],
                    ['x', 'r'],
                    ['y', 'c'],
                    ['z', 'j'],
                ],
            },
            RotorType::II => Self {
                notch: 'm',
                position: (position + 65) as u8 as char,
                wiring: [
                    ['a', 'a'],
                    ['b', 'j'],
                    ['c', 'd'],
                    ['d', 'k'],
                    ['e', 's'],
                    ['f', 'i'],
                    ['g', 'r'],
                    ['h', 'u'],
                    ['i', 'x'],
                    ['j', 'b'],
                    ['k', 'l'],
                    ['l', 'h'],
                    ['m', 'w'],
                    ['n', 't'],
                    ['o', 'm'],
                    ['p', 'c'],
                    ['q', 'q'],
                    ['r', 'g'],
                    ['s', 'z'],
                    ['t', 'n'],
                    ['u', 'p'],
                    ['v', 'y'],
                    ['w', 'f'],
                    ['x', 'v'],
                    ['y', 'o'],
                    ['z', 'e'],
                ],
            },
            RotorType::III => Self {
                notch: 'd',
                position: (position + 65) as u8 as char,
                wiring: [
                    ['a', 'b'],
                    ['b', 'd'],
                    ['c', 'f'],
                    ['d', 'h'],
                    ['e', 'j'],
                    ['f', 'l'],
                    ['g', 'c'],
                    ['h', 'p'],
                    ['i', 'r'],
                    ['j', 't'],
                    ['k', 'x'],
                    ['l', 'v'],
                    ['m', 'z'],
                    ['n', 'n'],
                    ['o', 'y'],
                    ['p', 'e'],
                    ['q', 'i'],
                    ['r', 'w'],
                    ['s', 'g'],
                    ['t', 'a'],
                    ['u', 'k'],
                    ['v', 'm'],
                    ['w', 'u'],
                    ['x', 's'],
                    ['y', 'q'],
                    ['z', 'o'],
                ],
            },
            RotorType::IV => Self {
                notch: 'r',
                position: (position + 65) as u8 as char,
                wiring: [
                    ['a', 'e'],
                    ['b', 's'],
                    ['c', 'o'],
                    ['d', 'v'],
                    ['e', 'p'],
                    ['f', 'z'],
                    ['g', 'j'],
                    ['h', 'a'],
                    ['i', 'y'],
                    ['j', 'q'],
                    ['k', 'u'],
                    ['l', 'i'],
                    ['m', 'r'],
                    ['n', 'h'],
                    ['o', 'x'],
                    ['p', 'l'],
                    ['q', 'n'],
                    ['r', 'f'],
                    ['s', 't'],
                    ['t', 'g'],
                    ['u', 'k'],
                    ['v', 'd'],
                    ['w', 'c'],
                    ['x', 'm'],
                    ['y', 'w'],
                    ['z', 'b'],
                ],
            },
            RotorType::V => Self {
                notch: 'h',
                position: (position + 65) as u8 as char,
                wiring: [
                    ['a', 'v'],
                    ['b', 'z'],
                    ['c', 'b'],
                    ['d', 'r'],
                    ['e', 'g'],
                    ['f', 'i'],
                    ['g', 't'],
                    ['h', 'y'],
                    ['i', 'u'],
                    ['j', 'p'],
                    ['k', 's'],
                    ['l', 'd'],
                    ['m', 'n'],
                    ['n', 'h'],
                    ['o', 'l'],
                    ['p', 'x'],
                    ['q', 'a'],
                    ['r', 'w'],
                    ['s', 'm'],
                    ['t', 'j'],
                    ['u', 'q'],
                    ['v', 'o'],
                    ['w', 'f'],
                    ['x', 'e'],
                    ['y', 'c'],
                    ['z', 'k'],
                ],
            },
        }
    }

    // Move data through the rotor forward
    pub fn forward(&self, input: char) -> char {
        let mut output = input;
        for i in 0..26 {
            if self.wiring[i][0] == input {
                output = self.wiring[i][1];
                break;
            }
        }
        output
    }

    // Move data through the rotor backward
    pub fn backward(&self, input: char) -> char {
        let mut output = input;
        for i in 0..26 {
            if self.wiring[i][1] == input {
                output = self.wiring[i][0];
                break;
            }
        }
        output
    }

    // Rotate the rotor
    pub fn rotate(&mut self) {
        let temp = self.wiring[0];
        for i in 0..25 {
            self.wiring[i] = self.wiring[i + 1];
        }
        self.wiring[25] = temp;
    }

    // Check if the rotor is at the notch
    pub fn is_notch(&self) -> bool {
        self.position == self.notch
    }
}
