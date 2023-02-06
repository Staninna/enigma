use crate::input::Input;

pub struct Reflector {
    wiring: [[Input; 2]; 13],
}

impl Reflector {
    pub fn new(reflector_type: ReflectorType) -> Reflector {
        let wiring = match reflector_type {
            ReflectorType::B => [
                [Input::Y, Input::R],
                [Input::U, Input::H],
                [Input::Q, Input::S],
                [Input::L, Input::D],
                [Input::P, Input::X],
                [Input::N, Input::G],
                [Input::O, Input::K],
                [Input::M, Input::I],
                [Input::E, Input::B],
                [Input::F, Input::Z],
                [Input::C, Input::W],
                [Input::V, Input::J],
                [Input::A, Input::T],
            ],
            ReflectorType::C => [
                [Input::F, Input::V],
                [Input::P, Input::J],
                [Input::A, Input::I],
                [Input::O, Input::Y],
                [Input::E, Input::D],
                [Input::R, Input::Z],
                [Input::X, Input::W],
                [Input::G, Input::C],
                [Input::T, Input::K],
                [Input::U, Input::Q],
                [Input::S, Input::B],
                [Input::N, Input::M],
                [Input::H, Input::L],
            ],
        };
        Reflector { wiring }
    }

    pub fn reflect(&self, input: Input) -> Input {
        let mut output = input;
        for i in 0..13 {
            if self.wiring[i][0] == input {
                output = self.wiring[i][1];
            } else if self.wiring[i][1] == input {
                output = self.wiring[i][0];
            }
        }
        output
    }
}

#[allow(dead_code)]
pub enum ReflectorType {
    B,
    C,
}
