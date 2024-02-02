use crate::visual_sort::SortBase;
use crate::{BoostrapColor, VisualSort};
use leptos::*;

pub struct Selection {
    base: SortBase,
    s: usize,
    x: usize,
    y: usize,
}

impl VisualSort for Selection {
    fn new(base: SortBase) -> Self {
        Self {
            base,
            s: 0,
            x: 0,
            y: 0,
        }
    }

    fn done(&self) -> bool {
        self.base.done
    }

    fn draw(&mut self, ticks: usize) {
        for _ in 0..ticks {
            self.update();
        }

        self.base.draw(|done: bool, i: usize| {
            if !done && i == self.y {
                BoostrapColor::Light.as_str()
            } else if !done && (i == self.x || i == self.s) {
                BoostrapColor::Green.as_str()
            } else {
                BoostrapColor::Red.as_str()
            }
        });
    }

    fn osc_stop(&self) {
        let _ = self.base.osc.stop();
    }

    fn update(&mut self) {
        if self.x < self.base.data.len() - 1 {
            if self.y < self.base.data.len() {
                self.base.array_access.update(|n| *n += 1);
                if self.base.data[self.y] < self.base.data[self.s] {
                    self.s = self.y;
                    self.base.set_freq(self.base.data[self.s]);
                }
                self.y += 1;
                return;
            }

            self.base.array_swap.update(|n| *n += 1);
            self.base.data.swap(self.x, self.s);

            self.x += 1;
            self.s = self.x;
            self.y = self.x + 1;
            self.base.set_freq(self.base.data[self.s]);
        } else {
            self.base.done = true;
        }
    }
}
