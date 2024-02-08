use super::{SortBase, VisualSort};
use crate::BoostrapColor;
use leptos::*;

pub struct Radix {
    base: SortBase,
    x: usize,
    y: usize,
}

impl VisualSort for Radix {
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
            // TODO
            if !done && i == self.y + 1 {
                BoostrapColor::Light.as_str()
            } else {
                BoostrapColor::Red.as_str()
            }
        });
    }

    fn osc_stop(&self) {
        let _ = self.base.osc.stop();
    }

    fn update(&mut self) {
        // TODO
    }
}
