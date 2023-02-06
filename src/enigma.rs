use crate::{input::Input, plugboard::Plugboard, reflector::Reflector, rotor::Rotor};

pub struct Enigma {
    rotor1: Rotor,
    rotor2: Rotor,
    rotor3: Rotor,
    reflector: Reflector,
    plugboard: Plugboard,
}

impl Enigma {
    pub fn new(
        rotor1: Rotor,
        rotor2: Rotor,
        rotor3: Rotor,
        reflector: Reflector,
        plugboard: Plugboard,
    ) -> Enigma {
        Enigma {
            rotor1,
            rotor2,
            rotor3,
            reflector,
            plugboard,
        }
    }

    pub fn input(&mut self, input: Input) -> Input {
        let mut output = input;
        output = self.plugboard.forward(output);

        output = self.rotor1.forward(output);
        output = self.rotor2.forward(output);
        output = self.rotor3.forward(output);

        output = self.reflector.reflect(output);

        output = self.rotor3.backward(output);
        output = self.rotor2.backward(output);
        output = self.rotor1.backward(output);

        output = self.plugboard.forward(output);

        self.rotate();

        output
    }

    fn rotate(&mut self) -> () {
        self.rotor1.rotate();
        if self.rotor1.notch == self.rotor2.notch {
            self.rotor2.rotate();
        }

        if self.rotor2.notch == self.rotor3.notch {
            self.rotor3.rotate();
        }
    }
}
