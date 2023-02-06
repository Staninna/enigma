pub struct Plug {
    pub contacts: [char; 2],
}

impl Plug {
    pub fn new(contact_1: char, contact_2: char) -> Self {
        Self {
            contacts: [contact_1, contact_2],
        }
    }

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
