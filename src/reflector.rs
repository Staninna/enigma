// All possible reflector types
#[allow(dead_code)]
pub enum ReflectorType {
    B,
    C,
}

// A reflector
pub struct Reflector {
    wiring: [[char; 2]; 26],
}

impl Reflector {
    // Create a new reflector based on given reflector type
    pub fn new(reflector_type: ReflectorType) -> Self {
        match reflector_type {
            ReflectorType::B => Self {
                wiring: [
                    ['a', 'y'],
                    ['b', 'r'],
                    ['c', 'u'],
                    ['d', 'h'],
                    ['e', 'q'],
                    ['f', 's'],
                    ['g', 'l'],
                    ['h', 'd'],
                    ['i', 'p'],
                    ['j', 'x'],
                    ['k', 'n'],
                    ['l', 'g'],
                    ['m', 'o'],
                    ['n', 'k'],
                    ['o', 'm'],
                    ['p', 'i'],
                    ['q', 'e'],
                    ['r', 'b'],
                    ['s', 'f'],
                    ['t', 'z'],
                    ['u', 'c'],
                    ['v', 'w'],
                    ['w', 'v'],
                    ['x', 'j'],
                    ['y', 'a'],
                    ['z', 't'],
                ],
            },
            ReflectorType::C => Self {
                wiring: [
                    ['a', 'f'],
                    ['b', 'v'],
                    ['c', 'p'],
                    ['d', 'j'],
                    ['e', 'i'],
                    ['f', 'a'],
                    ['g', 'o'],
                    ['h', 'y'],
                    ['i', 'e'],
                    ['j', 'd'],
                    ['k', 'r'],
                    ['l', 'z'],
                    ['m', 'x'],
                    ['n', 'w'],
                    ['o', 'g'],
                    ['p', 'c'],
                    ['q', 't'],
                    ['r', 'k'],
                    ['s', 'u'],
                    ['t', 'q'],
                    ['u', 's'],
                    ['v', 'b'],
                    ['w', 'n'],
                    ['x', 'm'],
                    ['y', 'h'],
                    ['z', 'l'],
                ],
            },
        }
    }

    // Move data through the reflector
    pub fn reflect(&self, data: char) -> char {
        for i in 0..26 {
            if data == self.wiring[i][0] {
                return self.wiring[i][1];
            } else if data == self.wiring[i][1] {
                return self.wiring[i][0];
            }
        }
        unreachable!("Invalid data passed to reflector");
    }
}
