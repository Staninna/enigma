mod enigma;
mod input;
mod plug;
mod plugboard;
mod reflector;
mod rotor;

use crate::{
    enigma::Enigma,
    input::Input,
    plug::Plug,
    plugboard::Plugboard,
    reflector::{Reflector, ReflectorType},
    rotor::{Rotor, RotorType},
};

fn main() {
    let mut enigma = Enigma::new(
        Rotor::new(1, RotorType::II),
        Rotor::new(10, RotorType::V),
        Rotor::new(15, RotorType::III),
        Reflector::new(ReflectorType::B),
        Plugboard::new(vec![
            Plug::new(Input::A, Input::Y),
            Plug::new(Input::V, Input::R),
            Plug::new(Input::U, Input::H),
            Plug::new(Input::Q, Input::E),
            Plug::new(Input::S, Input::F),
            Plug::new(Input::L, Input::G),
        ]),
    );

    let input_string = "ligma balls".to_uppercase().replace(" ", "");
    let mut output_string = String::new();
    for c in input_string.chars() {
        let input = Input::from_char(c);
        let output = enigma.input(input);
        output_string.push(output.to_char());
    }
    println!("{} -> {}", input_string, output_string);
}
