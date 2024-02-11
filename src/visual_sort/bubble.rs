use super::{Color, SortBase, VisualSort};
use leptos::*;

pub struct Bubble {
    base: SortBase,
    x: usize,
    y: usize,
}

impl VisualSort for Bubble {
    fn new(base: SortBase) -> Self {
        Self { base, x: 0, y: 0 }
    }

    fn done(&self) -> bool {
        self.base.done
    }

    fn draw(&mut self, ticks: usize) {
        for _ in 0..ticks {
            self.update();
        }

        self.base.draw(|done: bool, i: usize| {
            if !done && i == self.y + 1 {
                Color::Light.as_str()
            } else {
                Color::Red.as_str()
            }
        });
    }

    fn osc_stop(&self) {
        let _ = self.base.osc.stop();
    }

    fn update(&mut self) {
        for x in self.x..self.base.data.len() {
            self.x = x;
            for y in self.y..self.base.data.len() - x - 1 {
                self.y = y;
                self.base.array_access.update(|n| *n += 1);
                if self.base.data[y] > self.base.data[y + 1] {
                    self.base.data.swap(y, y + 1);
                    self.base.array_swap.update(|n| *n += 1);
                    self.base.set_freq(self.base.data[y + 1]);
                    return;
                }
            }
            self.y = 0;
        }
        self.base.done = true;
    }
}
