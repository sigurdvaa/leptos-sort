use crate::visual_sort::SortBase;
use crate::{BoostrapColor, VisualSort};
// use leptos::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
struct MergeState {
    left: Vec<usize>,
    right: Vec<usize>,
    l: usize,
    r: usize,
    sorted: bool,
}

pub struct Merge {
    base: SortBase,
    stack: Vec<MergeState>,
}

impl VisualSort for Merge {
    fn new(base: SortBase) -> Self {
        let len = base.data.len();
        let mid = len / 2;
        let left = (0..mid).collect();
        let right = (mid..len).collect();
        Self {
            base,
            stack: vec![MergeState {
                left,
                right,
                l: 0,
                r: 0,
                sorted: false,
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

        self.base
            .draw(|_done: bool, _i: usize| BoostrapColor::Red.as_str());
    }

    fn osc_stop(&self) {
        let _ = self.base.osc.stop();
    }

    fn update(&mut self) {
        while let Some(mut state) = self.stack.pop() {
            if state.left.is_empty() || state.right.is_empty() {
                continue;
            }
            log::info!("state: left: {:?} right {:?}", state.left, state.right);

            if !state.sorted {
                let (state_l, state_r) = self.split_state(&state);
                state.sorted = true;
                self.stack.push(state);
                self.stack.push(state_r);
                self.stack.push(state_l);
                continue;
            }

            let data = &mut self.base.data;
            while state.l < state.left.len() && state.r < state.right.len() {
                let l = state.left[state.l];
                let r = state.right[state.r];
                log::info!("cmp indexes {} and {}", l, r);
                if data[l] > data[r] {
                    log::info!("swapping {:?} and {:?}", data[l], data[r]);
                    data.swap(l, r);
                    // data.swap(l, r);
                    // state.r += 1;
                    // state.l += 1;
                }
                state.l += 1;
            }
        }

        self.base.done = true;
    }
}

impl Merge {
    fn split_state(&self, state: &MergeState) -> (MergeState, MergeState) {
        let mid = state.left.len() / 2;
        let left = state.left[..mid].to_owned();
        let right = state.left[mid..].to_owned();
        let state_l = MergeState {
            left,
            right,
            l: 0,
            r: 0,
            sorted: false,
        };

        let mid = state.right.len() / 2;
        let left = state.right[..mid].to_owned();
        let right = state.right[mid..].to_owned();
        let state_r = MergeState {
            left,
            right,
            l: 0,
            r: 0,
            sorted: false,
        };

        (state_l, state_r)
    }
}

// impl Merge {
//     fn merge_sort(&mut self, arr: &'a mut [usize]) {
//         // base case
//         if arr.len() <= 1 {
//             return;
//         }
//
//         // split arr in two
//         let mid = arr.len() / 2;
//         let mut arr_l = arr[..mid].to_owned();
//         let mut arr_r = arr[mid..].to_owned();
//
//         // sort sub arrays
//         self.merge_sort(&mut arr_l);
//         self.merge_sort(&mut arr_r);
//
//         // merge
//         self.merge_join(arr, &arr_l, &arr_r);
//     }
//     fn merge_join(&mut self, arr: &mut [usize], arr_l: &[usize], arr_r: &[usize]) {
//         let mut l = 0;
//         let mut r = 0;
//
//         while l < arr_l.len() && r < arr_r.len() {
//             if arr_l[l] < arr_r[r] {
//                 arr[l + r] = arr_l[l];
//                 l += 1;
//             } else {
//                 arr[l + r] = arr_r[r];
//                 r += 1;
//             }
//         }
//
//         while l < arr_l.len() {
//             arr[l + r] = arr_l[l];
//             l += 1;
//         }
//
//         while r < arr_r.len() {
//             arr[l + r] = arr_r[r];
//             r += 1;
//         }
//     }
// }

/*
        [6,8,6,9,5]
        [5,6,6,8,9]

        [5,6,6,8,9]-
      [0,1]    [2,3,4]-
    [0]  [1]   [2]  [3,4]-
                   [3] [4]



*/
