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
pub struct Rotor {
    notch: char,
    pub position: char,
    wiring: [[char; 2]; 26],
}

impl Rotor {
    // Create a new rotor based on given rotor type and position
    pub fn new(rotor_type: RotorType, position: u8) -> Self {
        match rotor_type {
            RotorType::I => Self {
                notch: 'y',
                position: (position + 97) as u8 as char,
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
                position: (position + 97) as u8 as char,
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
                position: (position + 97) as u8 as char,
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
                position: (position + 97) as u8 as char,
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
                position: (position + 97) as u8 as char,
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

    // Move data forward through the rotor
    pub fn forward(&self, data: char) -> char {
        let mut index = 0;
        for i in self.wiring.iter() {
            if i[0] == data {
                break;
            }
            index += 1;
        }
        self.wiring[index][1]
    }

    // Move data backwards through the rotor
    pub fn backward(&self, data: char) -> char {
        let mut index = 0;
        for i in self.wiring.iter() {
            if i[1] == data {
                break;
            }
            index += 1;
        }
        self.wiring[index][0]
    }

    // Rotate the rotor
    pub fn rotate(&mut self) {
        // Rotate the position
        self.position = (self.position as u8 + 1) as char;
        if self.position > 'z' {
            self.position = 'a';
        }

        // Rotate the wiring
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
