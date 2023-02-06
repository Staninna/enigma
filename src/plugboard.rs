use crate::{input::Input, plug::Plug};

pub struct Plugboard {
    pub wiring: Vec<Plug>,
}

impl Plugboard {
    pub fn new(wiring: Vec<Plug>) -> Plugboard {
        Plugboard { wiring }
    }

    pub fn forward(&self, input: Input) -> Input {
        let mut output = input;
        for plug in &self.wiring {
            if output == plug.plug1 {
                output = plug.plug2;
            } else if output == plug.plug2 {
                output = plug.plug1;
            }
        }
        output
    }
}
