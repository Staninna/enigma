pub struct PlugBoard {
    plugs: Vec<crate::plug::Plug>,
}

impl PlugBoard {
    pub fn new() -> Self {
        Self { plugs: vec![] }
    }

    pub fn add_plug(&mut self, plug: crate::plug::Plug) {
        for p in &self.plugs {
            if p.contacts[0] == plug.contacts[0] || p.contacts[0] == plug.contacts[1] {
                panic!(
                    "Plugboard already has a plug with contact `{}`",
                    plug.contacts[0]
                );
            }
            if p.contacts[1] == plug.contacts[0] || p.contacts[1] == plug.contacts[1] {
                panic!(
                    "Plugboard already has a plug with contact `{}`",
                    plug.contacts[1]
                );
            }
        }

        self.plugs.push(plug);
    }

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
