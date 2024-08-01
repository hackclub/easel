use anyhow::{bail, Result};
use fateful_peripheral::{peripheral, Peripheral};

#[peripheral(name = b"Multi-port Example")]
struct QuadRegister {
    reg: u32,
}

impl Peripheral for QuadRegister {
    fn init(ports: u8) -> Result<Self> {
        if ports != 4 {
            bail!("expected `4` connected ports, found `{ports}`");
        }
        Ok(QuadRegister { reg: 0 })
    }

    fn read(&mut self, port: u8) -> u8 {
        ((self.reg >> 8 * port) & 0xFF) as u8
    }

    fn write(&mut self, port: u8, data: u8) {
        let open = self.reg & !(0xFF << 8 * port);
        self.reg = open | ((data as u32) << 8 * port);
    }

    fn reset(&mut self) {
        self.reg = 0;
    }
}
