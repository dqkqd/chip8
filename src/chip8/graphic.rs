use crate::ui::display::Display;
use anyhow::Result;

const HEIGHT: usize = 32;
const WIDTH: usize = 64;
const SCALE: usize = 20;

pub(crate) struct Graphic {
    inner: [[u8; WIDTH]; HEIGHT],
}

impl Default for Graphic {
    fn default() -> Graphic {
        Graphic {
            inner: [[0; WIDTH]; HEIGHT],
        }
    }
}

impl Graphic {
    pub fn clear(&mut self) {
        *self = Graphic::default();
    }

    pub fn draw(&mut self, vx: usize, vy: usize, bitmap: &[[u8; 8]]) -> bool {
        let vx = vx % WIDTH;
        let vy = vy % HEIGHT;

        let mut turn_off = false;

        for dy in 0..bitmap.len() {
            let y = vy + dy;
            if y >= HEIGHT {
                break;
            }

            for dx in 0..8 {
                let x = vx + dx;
                if x >= WIDTH {
                    break;
                }
                let bit = &mut self.inner[y][x];
                if bit == &1 && bitmap[dy][dx] == 1 {
                    turn_off = true;
                }
                *bit ^= bitmap[dy][dx];
            }
        }

        turn_off
    }

    pub fn render(&mut self, display: &mut Display) -> Result<()> {
        display.render(&self.inner)
    }
}
