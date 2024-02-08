use super::{SortBase, VisualSort};
use crate::BoostrapColor;
use leptos::*;

pub struct Counting {
    base: SortBase,
    count: Vec<usize>,
    counted: bool,
    max: usize,
    maxed: bool,
    v: usize,
    x: usize,
}

impl VisualSort for Counting {
    fn new(base: SortBase) -> Self {
        Self {
            base,
            count: Vec::new(),
            counted: false,
            max: 0,
            maxed: false,
            v: 0,
            x: 0,
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
            if !done && i == self.x.saturating_sub(1) {
                if !self.maxed || !self.counted {
                    BoostrapColor::Light.as_str()
                } else {
                    BoostrapColor::Green.as_str()
                }
            } else {
                BoostrapColor::Red.as_str()
            }
        });
    }

    fn osc_stop(&self) {
        let _ = self.base.osc.stop();
    }

    fn update(&mut self) {
        // find max value
        if !self.maxed {
            self.base.array_access.update(|n| *n += 1);
            if self.base.data[self.x] > self.max {
                self.max = self.base.data[self.x];
                self.base.set_freq(self.max);
            }
            self.x += 1;
            if self.x < self.base.data.len() {
                return;
            }
            self.x = 0;
            self.maxed = true;
            self.count.resize(self.max + 1, 0)
        }

        // count values from 0 to max
        if !self.counted {
            self.base.array_access.update(|n| *n += 1);
            self.count[self.base.data[self.x]] += 1;
            self.base.set_freq(self.base.data[self.x]);
            self.x += 1;
            if self.x < self.base.data.len() {
                return;
            }
            self.x = 0;
            self.counted = true;
        }

        // update data based on count results
        if self.x < self.base.data.len() {
            while self.v < self.count.len() && self.count[self.v] == 0 {
                self.base.array_access.update(|n| *n += 1);
                self.v += 1;
            }
            self.count[self.v] -= 1;
            self.base.array_swap.update(|n| *n += 1);
            self.base.data[self.x] = self.v;
            self.base.set_freq(self.v);
            self.x += 1;
            return;
        }

        self.base.done = true;
    }
}
