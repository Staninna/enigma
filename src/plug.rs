pub struct Plug {
    pub contacts: [char; 2],
}

impl Plug {
    // Create a new plug
    pub fn new(contact_1: char, contact_2: char) -> Self {
        Self {
            contacts: [contact_1, contact_2],
        }
    }

    // Move data through the plug
    pub fn forward(&self, data: char) -> char {
        if data == self.contacts[0] {
            self.contacts[1]
        } else if data == self.contacts[1] {
            self.contacts[0]
        } else {
            data
        }
    }
}
