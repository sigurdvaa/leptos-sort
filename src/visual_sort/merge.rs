use super::{Color, SortBase, VisualSort};
use leptos::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
struct MergeState {
    arr: Rc<RefCell<Vec<usize>>>,
    arr_l: Rc<RefCell<Vec<usize>>>,
    arr_r: Rc<RefCell<Vec<usize>>>,
    l: usize,
    r: usize,
    s: usize,
    sorted: bool,
    start_i: usize,
}

pub struct Merge {
    base: SortBase,
    stack: Vec<MergeState>,
}

impl VisualSort for Merge {
    fn new(base: SortBase) -> Self {
        let arr = Rc::new(RefCell::new(base.data.clone()));
        let mid = arr.borrow().len() / 2;
        let arr_l = Rc::new(RefCell::new(arr.borrow()[..mid].to_owned()));
        let arr_r = Rc::new(RefCell::new(arr.borrow()[mid..].to_owned()));
        Self {
            base,
            stack: vec![MergeState {
                arr,
                arr_l,
                arr_r,
                l: 0,
                r: 0,
                s: 0,
                sorted: false,
                start_i: 0,
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

        match self.stack.last() {
            None => self.base.draw(|_done: bool, _i: usize| Color::Red.as_str()),
            Some(state) => self.base.draw(move |done: bool, i: usize| {
                if !done && i == (state.start_i + state.s).saturating_sub(1) {
                    Color::Light.as_str()
                } else if !done
                    && (i == state.start_i
                        || i == state.start_i + state.arr.borrow().len().saturating_sub(1))
                {
                    Color::Green.as_str()
                } else {
                    Color::Red.as_str()
                }
            }),
        }
    }

    fn osc_stop(&self) {
        let _ = self.base.osc.stop();
    }

    fn update(&mut self) {
        while let Some(mut state) = self.stack.pop() {
            if state.arr.borrow().len() <= 1 {
                continue;
            }

            if !state.sorted {
                let (left, right) = self.split_state(&state);
                state.sorted = true;
                self.stack.push(state);
                self.stack.push(right);
                self.stack.push(left);
                continue;
            }

            // scope for refcell borrow
            {
                let mut arr = state.arr.borrow_mut();
                let arr_l = state.arr_l.borrow();
                let arr_r = state.arr_r.borrow();
                if state.l < arr_l.len() && state.r < arr_r.len() {
                    self.base.array_access.update(|n| *n += 2);
                    self.base.array_cmp.update(|n| *n += 1);
                    self.base.array_swap.update(|n| *n += 1);
                    if arr_l[state.l] < arr_r[state.r] {
                        arr[state.l + state.r] = arr_l[state.l];
                        state.l += 1;
                    } else {
                        arr[state.l + state.r] = arr_r[state.r];
                        state.r += 1;
                    }
                } else if state.l < arr_l.len() {
                    self.base.array_swap.update(|n| *n += 1);
                    arr[state.l + state.r] = arr_l[state.l];
                    state.l += 1;
                } else if state.r < arr_r.len() {
                    self.base.array_swap.update(|n| *n += 1);
                    arr[state.l + state.r] = arr_r[state.r];
                    state.r += 1;
                }
            }

            if state.s < state.arr.borrow().len() {
                self.base.array_access.update(|n| *n += 1);
                self.base.array_swap.update(|n| *n += 1);
                let value = state.arr.borrow()[state.s];
                self.base.set_freq(value);
                self.base.data[state.start_i + state.s] = value;
                state.s += 1;
                self.stack.push(state);
                return;
            }
        }
        self.base.done = true;
    }
}

impl Merge {
    fn split_state(&self, state: &MergeState) -> (MergeState, MergeState) {
        let arr = state.arr_l.clone();
        let mid = arr.borrow().len() / 2;
        let arr_l = Rc::new(RefCell::new(arr.borrow()[..mid].to_owned()));
        let arr_r = Rc::new(RefCell::new(arr.borrow()[mid..].to_owned()));
        let left = MergeState {
            arr,
            arr_l,
            arr_r,
            l: 0,
            r: 0,
            s: 0,
            sorted: false,
            start_i: state.start_i,
        };

        let arr = state.arr_r.clone();
        let mid = arr.borrow().len() / 2;
        let arr_l = Rc::new(RefCell::new(arr.borrow()[..mid].to_owned()));
        let arr_r = Rc::new(RefCell::new(arr.borrow()[mid..].to_owned()));
        let right = MergeState {
            arr,
            arr_l,
            arr_r,
            l: 0,
            r: 0,
            s: 0,
            sorted: false,
            start_i: state.start_i + state.arr_l.borrow().len(),
        };

        (left, right)
    }
}
