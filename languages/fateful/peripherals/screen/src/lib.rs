use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread::{self, JoinHandle},
};

use anyhow::bail;
use fateful_peripheral::{peripheral, Peripheral};
use minifb::{Scale, Window, WindowOptions};

enum Event {
    Write { addr: u16, data: u8 },
    Quit,
    Reset,
}

#[peripheral(name = b"Screen")]
struct State {
    command: Option<u8>,
    addr_low: Option<u8>,
    addr_high: Option<u8>,
    tx: Sender<Event>,
    handle: JoinHandle<()>,
}

impl Peripheral for State {
    fn init(ports: u8) -> anyhow::Result<Self> {
        if ports != 1 {
            bail!("expected 1 port, found {ports}")
        }

        let (tx, rx) = channel();

        let handle = thread::spawn(move || window(rx));

        Ok(State {
            tx,
            handle,
            command: None,
            addr_low: None,
            addr_high: None,
        })
    }

    fn write(&mut self, _port: u8, data: u8) {
        match (self.command, self.addr_high, self.addr_low) {
            (None, None, None) => self.command = Some(data),
            (Some(0x10), None, None) => self.addr_high = Some(data),
            (Some(0x10), Some(_), None) => self.addr_low = Some(data),
            (Some(0x10), Some(high), Some(low)) => {
                self.command = None;
                self.addr_high = None;
                self.addr_low = None;
                self.tx
                    .send(Event::Write {
                        addr: ((high as u16) << 8) | (low as u16),
                        data,
                    })
                    .unwrap()
            }
            _ => {}
        }
    }

    fn reset(&mut self) {
        self.tx.send(Event::Reset).unwrap();
    }

    fn drop(self) {
        self.tx.send(Event::Quit).unwrap();
        self.handle.join().unwrap();
    }
}

fn window(rx: Receiver<Event>) {
    let mut window = Window::new(
        "F8ful Screen",
        160,
        120,
        WindowOptions {
            scale: Scale::X8,
            ..WindowOptions::default()
        },
    )
    .unwrap();
    let mut buffer = [0; 160 * 120];
    window.update_with_buffer(&buffer, 160, 120).unwrap();

    loop {
        let mut redraw = false;

        if let Ok(ev) = rx.try_recv() {
            match ev {
                Event::Write { addr, data } => {
                    let base = addr as usize * 8;
                    for i in 0..8 {
                        buffer[base + i] = if (data & 1 << i) != 0 { u32::MAX } else { 0 };
                    }

                    redraw = true;
                }
                Event::Quit => break,
                Event::Reset => {
                    buffer.fill(0);
                    redraw = true;
                }
            }
        }

        if redraw {
            window.update_with_buffer(&buffer, 160, 120).unwrap();
        } else {
            window.update();
        }
    }
}
