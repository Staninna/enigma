// A plugboard
#[derive(Clone)]
pub struct PlugBoard {
    plugs: Vec<crate::plug::Plug>,
}

impl PlugBoard {
    // Create a new plugboard
    pub fn new(plugs: &str) -> Self {
        // Normalize plugs
        let plugs = plugs.replace(" ", "").to_lowercase();

        // Check for invalid plugs
        for c in plugs.chars() {
            if !c.is_alphabetic() {
                panic!("Invalid plug: {}", c);
            }
        }

        // Check for even number of plugs
        if plugs.len() % 2 != 0 {
            panic!("Plugs must be in pairs");
        }

        // Check for duplicate plugs
        let mut plugs = plugs.chars().collect::<Vec<char>>();
        plugs.sort();
        for i in 0..plugs.len() - 1 {
            if plugs[i] == plugs[i + 1] {
                panic!("Duplicate plug: {}", plugs[i]);
            }
        }

        // Create the plugboard
        let mut plugboard = Self { plugs: Vec::new() };
        for i in 0..plugs.len() / 2 {
            plugboard
                .plugs
                .push(crate::plug::Plug::new(plugs[i * 2], plugs[i * 2 + 1]));
        }
        plugboard
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
