use super::{Color, SortBase, VisualSort};
use leptos::*;

pub struct Heap {
    base: SortBase,
    heap_len: usize,
    heapifying_down: bool,
    heapifying_up: bool,
    x: usize,
    y: usize,
}

impl VisualSort for Heap {
    fn new(base: SortBase) -> Self {
        Self {
            base,
            heap_len: 0,
            heapifying_down: false,
            heapifying_up: false,
            x: 0,
            y: 0,
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
            if !done && i == self.y {
                Color::Light.as_str()
            } else if !done && i == self.heap_len {
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
        // use self.base.data as initial unsorted items, heap, and sorted array

        // visualizing recursive heapify up
        if self.heapifying_up {
            self.heap_up(self.y);
            return;
        }

        // visualizing recursive heapify down
        if self.heapifying_down {
            self.heap_down(self.y);
            return;
        }

        // insert data to heap
        if self.x < self.base.data.len() {
            self.base.set_freq(self.base.data[self.x]);
            self.push(self.base.data[self.x]);
            self.x += 1;
            return;
        }

        // remove max from heap and insert to front of data (back of heap)
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
            self.base.array_access.update(|n| *n += 2);
            self.base.array_cmp.update(|n| *n += 1);
            if self.base.data[p] < self.base.data[i] {
                self.base.array_swap.update(|n| *n += 1);
                self.base.data.swap(p, i);
                self.heapifying_up = true;
                self.y = p;
                return;
            }
        }
        self.heapifying_up = false;
    }

    fn heap_down(&mut self, i: usize) {
        self.heapifying_down = false;
        let data = &self.base.data;
        let l = self.left_child(i);
        let r = self.right_child(i);
        let mut largest = i;

        self.base.array_access.update(|n| *n += 2);
        self.base.array_cmp.update(|n| *n += 1);
        if l < self.heap_len && data[l] > data[largest] {
            largest = l;
        }

        self.base.array_access.update(|n| *n += 2);
        self.base.array_cmp.update(|n| *n += 1);
        if r < self.heap_len && data[r] > data[largest] {
            largest = r;
        }

        if largest != i {
            self.base.array_swap.update(|n| *n += 1);
            self.base.data.swap(i, largest);
            self.heapifying_down = true;
            self.y = largest;
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

        self.base.array_access.update(|n| *n += 2);
        self.base.array_swap.update(|n| *n += 1);
        let value = Some(self.base.data[0]);
        self.heap_len -= 1;
        self.base.data[0] = self.base.data[self.heap_len];
        self.y = 0;
        self.heap_down(0);
        value
    }
}
