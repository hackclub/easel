use std::{
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender, TryRecvError},
        Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use anyhow::{bail, Result};
use fateful_peripheral::{peripheral, Peripheral};
use minifb::{Key, KeyRepeat, Window, WindowOptions};

const PORTS: u8 = 2;
const BIT_SIZE: usize = 100;
const WIDTH: usize = 8 * BIT_SIZE;
const HEIGHT: usize = 2 * BIT_SIZE;

static IN: Mutex<u8> = Mutex::new(0);
static OUT: Mutex<u8> = Mutex::new(0);

enum Event {
    Redraw,
    Stop,
}

#[peripheral(name = b"Digital I/O")]
struct State {
    events: SyncSender<Event>,
    handle: JoinHandle<()>,
}

impl Peripheral for State {
    fn init(ports: u8) -> Result<Self> {
        if ports != PORTS {
            bail!("expected `2` connected ports, found `{ports}`");
        }

        let (tx, rx) = sync_channel(16);

        let handle = thread::spawn(move || {
            background(rx);
        });

        Ok(State { handle, events: tx })
    }

    fn read(&mut self, port: u8) -> u8 {
        match port {
            0 => *IN.lock().unwrap(),
            1 => *OUT.lock().unwrap(),
            _ => unreachable!(),
        }
    }

    fn write(&mut self, port: u8, data: u8) {
        if port == 1 {
            *OUT.lock().unwrap() = data;
            self.events.send(Event::Redraw).unwrap();
        }
    }

    fn drop(self) {
        self.events.send(Event::Stop).unwrap();
        self.handle.join().expect("background thread has panicked");
    }
}

fn background(stop_rx: Receiver<Event>) {
    let mut buffer = [0; WIDTH * HEIGHT];

    let mut window = Window::new("Digital I/O", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    window.limit_update_rate(Some(Duration::from_micros(16600)));
    window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

    while window.is_open() {
        let mut redraw = match stop_rx.try_recv() {
            Ok(Event::Redraw) => true,
            Ok(Event::Stop) => return,
            Err(TryRecvError::Empty) => false,
            Err(TryRecvError::Disconnected) => return,
        };

        // Use key presses to toggle individual bits
        if window.is_key_pressed(Key::Key1, KeyRepeat::No) {
            *IN.lock().unwrap() ^= 0b0000_0001;
            redraw = true;
        }
        if window.is_key_pressed(Key::Key2, KeyRepeat::No) {
            *IN.lock().unwrap() ^= 0b0000_0010;
            redraw = true;
        }
        if window.is_key_pressed(Key::Key3, KeyRepeat::No) {
            *IN.lock().unwrap() ^= 0b0000_0100;
            redraw = true;
        }
        if window.is_key_pressed(Key::Key4, KeyRepeat::No) {
            *IN.lock().unwrap() ^= 0b0000_1000;
            redraw = true;
        }
        if window.is_key_pressed(Key::Key5, KeyRepeat::No) {
            *IN.lock().unwrap() ^= 0b0001_0000;
            redraw = true;
        }
        if window.is_key_pressed(Key::Key6, KeyRepeat::No) {
            *IN.lock().unwrap() ^= 0b0010_0000;
            redraw = true;
        }
        if window.is_key_pressed(Key::Key7, KeyRepeat::No) {
            *IN.lock().unwrap() ^= 0b0100_0000;
            redraw = true;
        }
        if window.is_key_pressed(Key::Key8, KeyRepeat::No) {
            *IN.lock().unwrap() ^= 0b1000_0000;
            redraw = true;
        }

        if redraw {
            for i in 0..8 {
                let color = if (*OUT.lock().unwrap() & (1 << (i as u8))) > 0 {
                    0xFFFFFF
                } else {
                    0x000000
                };

                rect(
                    &mut buffer,
                    WIDTH - (i * BIT_SIZE + BIT_SIZE),
                    0,
                    WIDTH - (i * BIT_SIZE) - 1,
                    BIT_SIZE - 1,
                    color,
                );
            }

            for i in 0..8 {
                let color = if (*IN.lock().unwrap() & (1 << (i as u8))) > 0 {
                    0xFFFFFF
                } else {
                    0x000000
                };

                rect(
                    &mut buffer,
                    WIDTH - (i * BIT_SIZE + BIT_SIZE),
                    BIT_SIZE,
                    WIDTH - (i * BIT_SIZE) - 1,
                    HEIGHT - 1,
                    color,
                );
            }

            window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        } else {
            window.update();
        }
    }
}

fn rect(buffer: &mut [u32], x1: usize, y1: usize, x2: usize, y2: usize, color: u32) {
    let i1 = y1 * WIDTH + x1;
    let i2 = y2 * WIDTH + x2;

    if i1 >= buffer.len() || i2 >= buffer.len() {
        return;
    }

    for y in y1..=y2 {
        let offset = y * WIDTH;
        for x in x1..=x2 {
            buffer[offset + x] = color;
        }
    }
}
