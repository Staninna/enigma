mod enigma;
mod plug;
mod plugboard;
mod reflector;
mod rotor;

use crate::{
    enigma::Enigma,
    plug::Plug,
    plugboard::PlugBoard,
    reflector::{Reflector, ReflectorType},
    rotor::{Rotor, RotorType},
};

fn main() {
    let mut enigma = Enigma::new(
        PlugBoard::new(),
        Reflector::new(ReflectorType::B),
        vec![
            Rotor::new(RotorType::I, 0),
            Rotor::new(RotorType::II, 10),
            Rotor::new(RotorType::III, 0),
        ],
    );

    // Add some random plugs
    enigma.add_plug(Plug::new('a', 'c'));
    enigma.add_plug(Plug::new('d', 't'));
    enigma.add_plug(Plug::new('y', 'u'));
    enigma.add_plug(Plug::new('i', 'o'));
    enigma.add_plug(Plug::new('w', 'p'));
    enigma.add_plug(Plug::new('q', 'r'));

    let result = enigma.send_string("");

    println!("{}", result);
}
