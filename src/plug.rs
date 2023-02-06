use crate::input::Input;

pub struct Plug {
    pub plug1: Input,
    pub plug2: Input,
}

impl Plug {
    pub fn new(plug1: Input, plug2: Input) -> Plug {
        Plug { plug1, plug2 }
    }
}
