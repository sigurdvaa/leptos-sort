use crate::visual_sort::SortBase;
use crate::{BoostrapColor, VisualSort};
use leptos::*;

pub struct Heap {
    base: SortBase,
    heaped: bool,
    heap_len: usize,
    x: usize,
    y: usize,
}

impl VisualSort for Heap {
    fn new(base: SortBase) -> Self {
        Self {
            base,
            heaped: false,
            heap_len: 0,
            x: 0,
            y: 0,
        }
    }

    fn done(&self) -> bool {
        self.base.done
    }

    fn draw(&mut self, ticks: usize) {
        // TODO: move to sortbase?
        for _ in 0..ticks {
            self.update();
        }

        let set_color = |done: bool, i: usize| {
            if !done && i == self.y {
                BoostrapColor::Light.as_str()
            } else if !done && i == self.heap_len {
                BoostrapColor::Green.as_str()
            } else {
                BoostrapColor::Red.as_str()
            }
        };

        self.base.draw(set_color);
    }

    fn osc_stop(&self) {
        let _ = self.base.osc.stop();
    }

    fn update(&mut self) {
        // use self.data as initial unsorted items, heap, and sorted array

        // add items to heap
        if !self.heaped {
            if self.x < self.base.data.len() {
                // TODO: if data len is greater than 450, the pitch is the same for all values
                self.base.set_freq(self.base.data[self.x]);
                self.push(self.base.data[self.x]);
                self.x += 1;
                return;
            }
            self.heaped = true;
        }

        // remove max heap, and add to last pos in array
        // TODO: visualize heapify?
        if let Some(v) = self.pop() {
            self.base.set_freq(v);
            self.base.array_swap.update(|n| *n += 1);
            self.base.data[self.heap_len] = v;
            return;
        }

        self.base.done = true;
    }
}

impl Heap {
    fn parent(&self, i: usize) -> Option<usize> {
        if i == 0 {
            return None;
        }
        Some((i - 1) / 2)
    }

    fn left_child(&self, i: usize) -> usize {
        i * 2 + 1
    }

    fn right_child(&self, i: usize) -> usize {
        i * 2 + 2
    }

    fn heap_up(&mut self, i: usize) {
        if let Some(p) = self.parent(i) {
            self.base.array_access.update(|n| *n += 1);
            if self.base.data[p] < self.base.data[i] {
                self.base.array_swap.update(|n| *n += 1);
                self.base.data.swap(p, i);
                self.heap_up(p);
                self.y = p;
            }
        }
    }

    fn heap_down(&mut self, i: usize) {
        self.y = i;
        let l = self.left_child(i);
        let r = self.right_child(i);

        if r > self.heap_len {
            return;
        }

        self.base.array_access.update(|n| *n += 1);
        if self.base.data[l] > self.base.data[r] && self.base.data[i] <= self.base.data[l] {
            self.base.array_swap.update(|n| *n += 1);
            self.base.data.swap(i, l);
            self.heap_down(l);
        } else if self.base.data[i] <= self.base.data[r] {
            self.base.array_swap.update(|n| *n += 1);
            self.base.data.swap(i, r);
            self.heap_down(r);
        }
    }

    fn push(&mut self, value: usize) {
        self.base.array_swap.update(|n| *n += 1);
        self.base.data[self.heap_len] = value;
        self.y = self.heap_len;
        self.heap_up(self.heap_len);
        self.heap_len += 1;
    }

    fn pop(&mut self) -> Option<usize> {
        if self.heap_len == 0 {
            return None;
        }

        let val = Some(self.base.data[0]);
        self.heap_len -= 1;
        self.base.data[0] = self.base.data[self.heap_len];
        self.heap_down(0);
        val
    }
}
