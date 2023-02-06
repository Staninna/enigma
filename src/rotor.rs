use crate::input::Input;

pub struct Rotor {
    pub position: Input,
    pub notch: Input,
    pub wiring: Vec<[Input; 2]>,
}

impl Rotor {
    pub fn new(position: u8, rotor_type: RotorType) -> Rotor {
        match rotor_type {
            RotorType::I => Rotor {
                position: Input::from(position),
                notch: Input::Y,
                wiring: vec![
                    [Input::A, Input::E],
                    [Input::B, Input::K],
                    [Input::C, Input::M],
                    [Input::D, Input::F],
                    [Input::E, Input::L],
                    [Input::F, Input::G],
                    [Input::G, Input::D],
                    [Input::H, Input::Q],
                    [Input::I, Input::V],
                    [Input::J, Input::Z],
                    [Input::K, Input::N],
                    [Input::L, Input::T],
                    [Input::M, Input::O],
                    [Input::N, Input::W],
                    [Input::O, Input::Y],
                    [Input::P, Input::H],
                    [Input::Q, Input::X],
                    [Input::R, Input::U],
                    [Input::S, Input::S],
                    [Input::T, Input::P],
                    [Input::U, Input::A],
                    [Input::V, Input::I],
                    [Input::W, Input::B],
                    [Input::X, Input::R],
                    [Input::Y, Input::C],
                    [Input::Z, Input::J],
                ],
            },
            RotorType::II => Rotor {
                position: Input::from(position),
                notch: Input::M,
                wiring: vec![
                    [Input::A, Input::A],
                    [Input::B, Input::J],
                    [Input::C, Input::D],
                    [Input::D, Input::K],
                    [Input::E, Input::S],
                    [Input::F, Input::I],
                    [Input::G, Input::R],
                    [Input::H, Input::U],
                    [Input::I, Input::X],
                    [Input::J, Input::B],
                    [Input::K, Input::L],
                    [Input::L, Input::H],
                    [Input::M, Input::W],
                    [Input::N, Input::T],
                    [Input::O, Input::M],
                    [Input::P, Input::C],
                    [Input::Q, Input::Q],
                    [Input::R, Input::G],
                    [Input::S, Input::Z],
                    [Input::T, Input::N],
                    [Input::U, Input::P],
                    [Input::V, Input::Y],
                    [Input::W, Input::F],
                    [Input::X, Input::V],
                    [Input::Y, Input::O],
                    [Input::Z, Input::E],
                ],
            },
            RotorType::III => Rotor {
                position: Input::from(position),
                notch: Input::D,
                wiring: vec![
                    [Input::A, Input::B],
                    [Input::B, Input::D],
                    [Input::C, Input::F],
                    [Input::D, Input::H],
                    [Input::E, Input::J],
                    [Input::F, Input::L],
                    [Input::G, Input::C],
                    [Input::H, Input::P],
                    [Input::I, Input::R],
                    [Input::J, Input::T],
                    [Input::K, Input::X],
                    [Input::L, Input::V],
                    [Input::M, Input::Z],
                    [Input::N, Input::N],
                    [Input::O, Input::Y],
                    [Input::P, Input::E],
                    [Input::Q, Input::I],
                    [Input::R, Input::W],
                    [Input::S, Input::G],
                    [Input::T, Input::A],
                    [Input::U, Input::K],
                    [Input::V, Input::M],
                    [Input::W, Input::U],
                    [Input::X, Input::S],
                    [Input::Y, Input::Q],
                    [Input::Z, Input::O],
                ],
            },
            RotorType::IV => Rotor {
                position: Input::from(position),
                notch: Input::R,
                wiring: vec![
                    [Input::A, Input::E],
                    [Input::B, Input::S],
                    [Input::C, Input::O],
                    [Input::D, Input::V],
                    [Input::E, Input::P],
                    [Input::F, Input::Z],
                    [Input::G, Input::J],
                    [Input::H, Input::A],
                    [Input::I, Input::Y],
                    [Input::J, Input::Q],
                    [Input::K, Input::U],
                    [Input::L, Input::I],
                    [Input::M, Input::R],
                    [Input::N, Input::H],
                    [Input::O, Input::X],
                    [Input::P, Input::L],
                    [Input::Q, Input::N],
                    [Input::R, Input::F],
                    [Input::S, Input::T],
                    [Input::T, Input::G],
                    [Input::U, Input::K],
                    [Input::V, Input::D],
                    [Input::W, Input::C],
                    [Input::X, Input::M],
                    [Input::Y, Input::W],
                    [Input::Z, Input::B],
                ],
            },
            RotorType::V => Rotor {
                position: Input::from(position),
                notch: Input::K,
                wiring: vec![
                    [Input::A, Input::V],
                    [Input::B, Input::Z],
                    [Input::C, Input::B],
                    [Input::D, Input::R],
                    [Input::E, Input::G],
                    [Input::F, Input::I],
                    [Input::G, Input::T],
                    [Input::H, Input::Y],
                    [Input::I, Input::U],
                    [Input::J, Input::P],
                    [Input::K, Input::S],
                    [Input::L, Input::D],
                    [Input::M, Input::N],
                    [Input::N, Input::H],
                    [Input::O, Input::L],
                    [Input::P, Input::X],
                    [Input::Q, Input::A],
                    [Input::R, Input::W],
                    [Input::S, Input::M],
                    [Input::T, Input::J],
                    [Input::U, Input::Q],
                    [Input::V, Input::O],
                    [Input::W, Input::F],
                    [Input::X, Input::E],
                    [Input::Y, Input::C],
                    [Input::Z, Input::K],
                ],
            },
        }
    }

    pub fn rotate(&mut self) {
        self.position = self.position.next();
    }

    pub fn forward(&self, input: Input) -> Input {
        let mut index = 0;
        for i in 0..self.wiring.len() {
            if self.wiring[i][0] == input {
                index = i;
                break;
            }
        }
        let mut output = self.wiring[index][1];
        for _ in 0..self.position as u8 {
            output = output.next();
        }
        output
    }

    pub fn backward(&self, input: Input) -> Input {
        let mut index = 0;
        for i in 0..self.wiring.len() {
            if self.wiring[i][1] == input {
                index = i;
                break;
            }
        }
        let mut output = self.wiring[index][0];
        for _ in 0..self.position as u8 {
            output = output.prev();
        }
        output
    }
}

#[allow(dead_code)]
pub enum RotorType {
    I,
    II,
    III,
    IV,
    V,
}
