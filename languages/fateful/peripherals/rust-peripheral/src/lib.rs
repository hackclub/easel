use anyhow::Result;
use fateful_peripheral::{peripheral, Peripheral};

#[peripheral(name = b"Rust Example")]
struct State {
    data: u8,
}

impl Peripheral for State {
    fn init(_ports: u8) -> Result<Self> {
        Ok(State { data: 0 })
    }

    fn read(&mut self, _port: u8) -> u8 {
        self.data
    }

    fn write(&mut self, _port: u8, data: u8) {
        self.data = data;
    }

    fn reset(&mut self) {
        self.data = 0;
    }
}
