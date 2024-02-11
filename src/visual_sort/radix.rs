use super::{Color, SortBase, VisualSort};
use leptos::*;

pub struct Radix {
    base: SortBase,
    count: [usize; 10],
    counted: bool,
    max: usize,
    maxed: bool,
    y: usize,
    x: usize,
    radix: u32,
    tmp_data: Vec<usize>,
}

impl VisualSort for Radix {
    fn new(base: SortBase) -> Self {
        let tmp_data = base.data.clone();
        Self {
            base,
            count: [0; 10],
            counted: false,
            max: 0,
            maxed: false,
            y: 0,
            x: 0,
            radix: 0,
            tmp_data,
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
                    Color::Light.as_str()
                } else {
                    Color::Green.as_str()
                }
            } else {
                Color::Red.as_str()
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
        }

        // count values from 0 to max
        if !self.counted {
            self.base.array_access.update(|n| *n += 1);
            let value = self.base.data[self.x];
            let base = value / 10_usize.pow(self.radix) % 10;
            self.count[base] += 1;
            self.base.set_freq(self.base.data[self.x]);
            self.x += 1;
            if self.x < self.base.data.len() {
                return;
            }
            self.x = 0;
            self.counted = true;
            for i in (0..self.count.len() - 1).rev() {
                self.count[i] += self.count[i + 1];
            }
        }

        // update data based on count results
        if self.y < self.tmp_data.len() {
            self.base.array_access.update(|n| *n += 1);
            let value = self.tmp_data[self.y];
            let base = value / 10_usize.pow(self.radix) % 10;
            self.base.array_access.update(|n| *n += 1);
            let i = self.tmp_data.len() - self.count[base];
            self.count[base] -= 1;
            self.base.array_swap.update(|n| *n += 1);
            self.base.data[i] = value;
            self.base.set_freq(value);
            self.x = i;
            self.y += 1;
            return;
        }

        // done if max < 10^radix
        if self.max >= 10_usize.pow(self.radix + 1) {
            self.x = 0;
            self.y = 0;
            self.count = [0; 10];
            self.counted = false;
            self.radix += 1;
            self.tmp_data = self.base.data.clone();
        } else {
            self.base.done = true
        }
    }
}
