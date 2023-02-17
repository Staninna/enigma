use std::fmt::Display;

// All possible reflector types
#[derive(Clone)]
pub enum ReflectorType {
    B,
    C,
}

// Convert a reflector type to a char
impl From<&ReflectorType> for char {
    fn from(reflector_type: &ReflectorType) -> Self {
        match reflector_type {
            ReflectorType::B => 'b',
            ReflectorType::C => 'c',
        }
    }
}

// Convert a char to a reflector type
impl From<char> for ReflectorType {
    fn from(reflector_type: char) -> Self {
        match reflector_type {
            'b' => ReflectorType::B,
            'c' => ReflectorType::C,
            _ => panic!("Invalid reflector type"),
        }
    }
}

impl Display for ReflectorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ReflectorType::B => write!(f, "b"),
            ReflectorType::C => write!(f, "c"),
        }
    }
}

// A reflector
#[derive(Clone)]
pub struct Reflector {
    pub reflector_type: ReflectorType,
    wiring: [[char; 2]; 26],
}

impl Reflector {
    // Create a new reflector based on given reflector type
    pub fn new(reflector_type: ReflectorType) -> Self {
        match reflector_type {
            ReflectorType::B => Self {
                reflector_type,
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
                reflector_type,
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

    // Create a random reflector
    pub fn random() -> Self {
        let reflector_type = match rand::random::<u8>() % 2 {
            0 => ReflectorType::B,
            1 => ReflectorType::C,
            _ => unreachable!(),
        };
        Self::new(reflector_type)
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
