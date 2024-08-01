use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::{pin::Pin, sync::atomic::AtomicBool};
use std::str::FromStr;

use async_std::task::JoinHandle;
use minifb::{Icon, Scale, ScaleMode, WindowOptions};

const FONT: &[u8; 1 << 12] = include_bytes!("../vga-font.rom");
const WIDTH: usize = 640;
const HEIGHT: usize = 400;

const COLORS: [u32; 16] = [
    0x000000,
    0x0000aa,
    0x00aa00,
    0x00aaaa,
    0xaa0000,
    0xaa00aa,
    0xaa5500,
    0xaaaaaa,
    0x555555,
    0x5555ff,
    0x55ff55,
    0x55ffff,
    0xff5555,
    0xff55ff,
    0xffff55,
    0xffffff,
];

#[derive(Debug)]
pub struct TextBuffer {
    chars: Pin<Box<[u8; 1 << 11]>>,
    modifiers: Pin<Box<[u8; 1 << 11]>>,
    modified: Arc<AtomicBool>,
    handle: JoinHandle<()>,
}

struct BufferPtr {
    chars: *const [u8; 1 << 11],
    modifiers: *const [u8; 1 << 11],
    modified: Arc<AtomicBool>,
}

unsafe impl Send for BufferPtr {}

impl TextBuffer {
    pub fn spawn() -> TextBuffer {
        let chars = Box::pin([0; 1 << 11]);
        let modifiers = Box::pin([0; 1 << 11]);
        let modified = Arc::new(AtomicBool::new(true));

        let handle = async_std::task::spawn(run_handle(BufferPtr{
            chars: &*chars,
            modifiers: &*modifiers,
            modified: modified.clone(),
        }));

        TextBuffer { chars, modifiers, modified, handle }
    }

    pub fn get(&self, addr: u16) -> u8 {
        if addr % 2 == 0 {
            let sub_index = (addr >> 1) as usize;
            self.chars[sub_index]
        } else {
            let sub_index = (addr >> 1) as usize;
            self.modifiers[sub_index]
        }
    }

    pub fn set(&mut self, addr: u16, data: u8) {
        self.modified.store(true, Ordering::Relaxed);

        if addr % 2 == 0 {
            let sub_index = (addr >> 1) as usize;
            self.chars[sub_index] = data;
        } else {
            let sub_index = (addr >> 1) as usize;
            self.modifiers[sub_index] = data;
        }
    }

    pub fn reset(&mut self) {
        self.chars.fill(0);
        self.modifiers.fill(0);
    }
}

async fn run_handle(buffer: BufferPtr) {
    let mut opts = WindowOptions::default();
    opts.scale = Scale::FitScreen;
    opts.scale_mode = ScaleMode::AspectRatioStretch;
    opts.resize = true;
    
    let mut window =
        minifb::Window::new("f8ful", WIDTH, HEIGHT, opts).expect("should be able to create window");
    // the actual VGA standard is 75, but this function is very resource intensive and should not run constantly
    window.set_target_fps(75);
    if let Some(icon) = get_icon() {
        window.set_icon(icon);
    }

    let mut fb = [0x00000000; WIDTH * HEIGHT];

    while window.is_open() {
        if buffer.modified.load(Ordering::Relaxed) {
            buffer.modified.store(false, Ordering::Relaxed);
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let font_x = x % 8;
                    let font_y = y % 16;

                    let char_x = x / 8;
                    let char_y = y / 16;
                    let char_idx = char_x + char_y * WIDTH / 8;
                    let (character, modifier) = unsafe {(
                        (*buffer.chars)[char_idx], 
                        (*buffer.modifiers)[char_idx]
                    )};

                    let font_addr = ((character as usize) << 4) + font_y;
                    let lit = FONT[font_addr] & (1 << (7 - font_x)) > 0;

                    // This part isn't part of the actual CPU,
                    // the real value will be transmitted via VGA instead of stored.
                    let fg = COLORS[(modifier & 0xf) as usize];
                    let bg = COLORS[(modifier >> 4) as usize];
                    fb[x + y * WIDTH] = if lit { fg } else { bg };
                }
            }

            window
                .update_with_buffer(&fb, WIDTH, HEIGHT)
                .expect("unable to write to window");
        } else {
            window.update();
        }
    }
}

fn get_icon() -> Option<Icon> {
    // TODO: actually add icon
    None
}
