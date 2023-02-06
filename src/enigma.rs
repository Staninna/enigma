use crate::{plug::Plug, plugboard::PlugBoard, reflector::Reflector, rotor::Rotor};

pub struct Enigma {
    plugboard: PlugBoard,
    reflector: Reflector,
    rotors: [Rotor; 3],
}

impl Enigma {
    pub fn new(plugboard: PlugBoard, reflector: Reflector, rotors: Vec<Rotor>) -> Self {
        Self {
            plugboard,
            reflector,
            rotors: [rotors[0], rotors[1], rotors[2]],
        }
    }

    pub fn add_plug(&mut self, plug: Plug) {
        self.plugboard.add_plug(plug);
    }

    pub fn send_string(&mut self, data: &str) -> String {
        let normalized_data = data.replace(" ", "").to_lowercase();
        let mut result = String::new();
        for c in normalized_data.chars() {
            result.push(self.send(c));
        }
        result
    }

    pub fn send(&mut self, data: char) -> char {
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
