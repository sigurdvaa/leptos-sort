use leptos::html::Canvas;
use leptos::*;

pub mod bubble;
pub mod insertion;
pub mod quick;

// pub use bubble::Bubble;
pub use insertion::Insertion;
pub use quick::Quick;

pub trait VisualSort {
    fn done(&self) -> bool;
    fn draw(&mut self, ticks: usize);
    fn osc_stop(&self);
}

pub enum Sort {
    Bubble,
    Insertion,
    Quick,
}

impl Sort {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Bubble => "/bubble",
            Self::Insertion => "/insertion",
            Self::Quick => "/quick",
            // TODO: select sort
            // TODO: merge sort
        }
    }
    pub fn new(
        &self,
        canvas_ref: &NodeRef<Canvas>,
        items: usize,
        volume: RwSignal<f32>,
        access: RwSignal<usize>,
        swap: RwSignal<usize>,
    ) -> impl VisualSort {
        match self {
            Self::Bubble => bubble::Bubble::new(canvas_ref, items, volume, access, swap),
            Self::Insertion => bubble::Bubble::new(canvas_ref, items, volume, access, swap),
            Self::Quick => bubble::Bubble::new(canvas_ref, items, volume, access, swap),
        }
    }
}
