// A plugboard
pub struct PlugBoard {
    plugs: Vec<crate::plug::Plug>,
}

impl PlugBoard {
    // Create a new plugboard
    pub fn new() -> Self {
        Self { plugs: vec![] }
    }

    // Add a plug to the plugboard
    pub fn add_plug(&mut self, plug: crate::plug::Plug) {
        self.plugs.push(plug);
    }

    // Move data through the plugboard
    pub fn move_data(&self, data: char) -> char {
        let mut result = data;
        for plug in &self.plugs {
            result = plug.forward(data);
            if result != data {
                break;
            }
        }
        result
    }
}
