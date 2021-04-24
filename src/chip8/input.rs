pub struct Input {
    current_key: Option<u8>,
}

impl Input {
    pub fn new() -> Input {
        Input { current_key: None }
    }

    pub fn set_current_key(&mut self, key: Option<u8>) {
        self.current_key = key;
    }

    pub fn get_current_key(&self) -> Option<u8> {
        self.current_key
    }
}
