use super::{SortBase, VisualSort};
use crate::BoostrapColor;
use leptos::*;

struct QuickState {
    lo: usize,
    hi: usize,
    pivot: usize,
    i: usize,
}

pub struct Quick {
    base: SortBase,
    pivots: Vec<QuickState>,
}

impl VisualSort for Quick {
    fn new(base: SortBase) -> Self {
        let hi = base.data.len() - 1;
        Self {
            base,
            pivots: vec![QuickState {
                lo: 0,
                hi,
                pivot: 0,
                i: 0,
            }],
        }
    }

    fn done(&self) -> bool {
        self.base.done
    }

    fn draw(&mut self, ticks: usize) {
        for _ in 0..ticks {
            self.update();
        }

        let (curr_pivot, curr_lo, curr_hi, curr_i) = match self.pivots.last() {
            Some(state) => (
                state.pivot.saturating_sub(1),
                state.lo,
                state.hi,
                state.i.saturating_sub(1),
            ),
            None => (0, 0, self.base.data.len() - 1, 0),
        };

        self.base.draw(|done: bool, i: usize| {
            if !done && (i == curr_hi || i == curr_lo) {
                BoostrapColor::Green.as_str()
            } else if !done && (i == curr_pivot || i == curr_i) {
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
        // continue previous state, or start a new lower or upper half
        let mut state = match self.pivots.pop() {
            Some(data) => data,
            None => {
                self.base.done = true;
                return;
            }
        };

        // find all less or equal to pivot, return on tick
        for i in state.i..state.hi {
            self.base.array_access.update(|n| *n += 1);
            if self.base.data[i] <= self.base.data[state.hi] {
                self.base.array_swap.update(|n| *n += 1);
                self.base.data.swap(i, state.pivot);
                self.base.set_freq(self.base.data[state.pivot]);
                // tick done
                state.pivot += 1;
                state.i = i + 1;
                self.pivots.push(state);
                return;
            }
        }

        // when all less or equal to pivot has been found
        //   move pivot to it's sorted position
        if state.pivot >= self.base.data.len() {
            state.pivot = self.base.data.len() - 1;
        }
        self.base.data.swap(state.hi, state.pivot);
        self.base.array_swap.update(|n| *n += 1);

        // add state for upper half of pivot
        if state.pivot + 1 < state.hi {
            self.pivots.push(QuickState {
                lo: state.pivot + 1,
                hi: state.hi,
                pivot: state.pivot + 1,
                i: state.pivot + 1,
            });
        }

        // add state for lower half of pivot
        if state.pivot > 0 && state.lo < state.pivot - 1 {
            self.pivots.push(QuickState {
                lo: state.lo,
                hi: state.pivot - 1,
                pivot: state.lo,
                i: state.lo,
            });
        }
    }
}
