use crate::ui::display::Display;
use anyhow::Result;

pub(crate) const HEIGHT: usize = 32;
pub(crate) const WIDTH: usize = 64;

pub(crate) struct Graphic {
    inner: [[u8; WIDTH]; HEIGHT],
    rerender: bool,
}

impl Default for Graphic {
    fn default() -> Graphic {
        Graphic {
            inner: [[0; WIDTH]; HEIGHT],
            rerender: false,
        }
    }
}

impl Graphic {
    pub fn clear(&mut self) {
        *self = Graphic::default();
    }

    pub fn draw(&mut self, vx: usize, vy: usize, bitmap: &[[u8; 8]]) -> bool {
        let lx = vx % WIDTH;
        let ly = vy % HEIGHT;
        let rx = (lx + 8).min(WIDTH);
        let ry = (ly + bitmap.len()).min(HEIGHT);

        let mut turn_off = false;
        for (old_row, new_row) in self.inner[ly..ry].iter_mut().zip(bitmap) {
            for (old_bit, new_bit) in old_row[lx..rx].iter_mut().zip(new_row) {
                if new_bit == &1 && old_bit == &1 {
                    turn_off = true;
                }
                *old_bit ^= new_bit;
            }
        }
        self.rerender = true;

        turn_off
    }

    pub fn render(&mut self, display: &mut Display) -> Result<()> {
        // no need to render
        if !self.rerender {
            return Ok(());
        }

        let point_locations = self
            .inner
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, pixel)| pixel == &&1)
                    .map(|(x, _)| (x, y))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        self.rerender = false;
        display.render(&point_locations)
    }
}
