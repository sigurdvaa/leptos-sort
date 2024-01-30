use leptos::html::Canvas;
use leptos::*;

mod bubble;
// mod insertion;
// mod quick;

pub trait VisualSort {
    fn new(
        canvas_ref: &NodeRef<Canvas>,
        items: usize,
        volume: RwSignal<f32>,
        access: RwSignal<usize>,
        swap: RwSignal<usize>,
    ) -> Self;

    fn done(&self) -> bool;

    fn draw(&mut self, ticks: usize);

    fn osc_stop(&self);

    fn update(&mut self);
}

pub enum Sort {
    Bubble,
    // Insertion,
    // Quick,
    // TODO: select sort
    // TODO: merge sort
}

impl Sort {
    pub fn name_as_str(&self) -> &'static str {
        match self {
            Self::Bubble => "Bubble Sort",
            // Self::Insertion => "Insertion Sort",
            // Self::Quick => "Quick Sort",
        }
    }

    pub fn route_as_str(&self) -> &'static str {
        match self {
            Self::Bubble => "/bubble",
            // Self::Insertion => "/insertion",
            // Self::Quick => "/quick",
        }
    }

    pub fn init(
        &self,
        canvas_ref: &NodeRef<Canvas>,
        items: usize,
        volume: RwSignal<f32>,
        access: RwSignal<usize>,
        swap: RwSignal<usize>,
    ) -> impl VisualSort {
        match self {
            Self::Bubble => bubble::Bubble::new(canvas_ref, items, volume, access, swap),
            // Self::Insertion => Box::new(insertion::Insertion::new(
            //     canvas_ref, items, volume, access, swap,
            // )),
            // Self::Quick => Box::new(quick::Quick::new(canvas_ref, items, volume, access, swap)),
        }
    }
}
