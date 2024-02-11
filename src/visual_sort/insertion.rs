use super::{Color, SortBase, VisualSort};
use leptos::*;

pub struct Insertion {
    base: SortBase,
    x: usize,
    y: usize,
    inserting: bool,
}

impl VisualSort for Insertion {
    fn new(base: SortBase) -> Self {
        Self {
            base,
            x: 1,
            y: 0,
            inserting: false,
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
            if !done && self.inserting && self.y == i {
                Color::Light.as_str()
            } else if !done && self.x - 1 == i {
                Color::Green.as_str()
            } else {
                Color::Red.as_str()
            }
        });
    }

    fn osc_stop(&self) {
        let _ = self.base.osc.stop();
    }

    fn update(&mut self) {
        if self.inserting {
            if self.y > 0 {
                let i = self.y - 1;
                self.base.array_access.update(|n| *n += 1);
                if self.base.data[self.y] < self.base.data[i] {
                    self.base.array_swap.update(|n| *n += 1);
                    self.base.data.swap(self.y, i);
                    self.y = i;
                    return;
                }
            }
            self.inserting = false;
        };

        for x in self.x..self.base.data.len() {
            self.x = x;
            let i = x - 1;
            self.base.array_access.update(|n| *n += 1);
            if self.base.data[x] < self.base.data[i] {
                self.base.data.swap(x, i);
                self.base.array_swap.update(|n| *n += 1);
                self.base.set_freq(self.base.data[i]);
                self.x = x + 1;
                self.inserting = true;
                self.y = i;
                return;
            }
        }
        self.base.done = true;
    }
}
