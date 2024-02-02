use crate::visual_sort::SortBase;
use crate::{BoostrapColor, VisualSort};
use leptos::*;

pub struct Merge {
    base: SortBase,
    x: usize,
    y: usize,
}

impl VisualSort for Merge {
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
        todo!();
    }
}
