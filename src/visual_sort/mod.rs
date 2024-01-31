use leptos::html::Canvas;
use leptos::*;

mod bubble;
mod insertion;
mod quick;
mod selection;

pub struct SortParams<'a> {
    pub canvas_ref: &'a NodeRef<Canvas>,
    pub items: usize,
    pub volume: RwSignal<f32>,
    pub array_access: RwSignal<usize>,
    pub array_swap: RwSignal<usize>,
}

pub trait VisualSort {
    fn new(params: SortParams) -> Self
    where
        Self: Sized;

    fn done(&self) -> bool;

    fn draw(&mut self, ticks: usize);

    fn osc_stop(&self);

    fn update(&mut self);
}

pub enum Sort {
    Bubble,
    Insertion,
    Quick,
    Selection,
    // TODO: merge sort
    // TODO: heapsort
}

impl Sort {
    pub fn name_as_str(&self) -> &'static str {
        match self {
            Self::Bubble => "Bubble Sort",
            Self::Insertion => "Insertion Sort",
            Self::Quick => "Quicksort",
            Self::Selection => "Selection Sort",
        }
    }

    pub fn route_as_str(&self) -> &'static str {
        match self {
            Self::Bubble => "/bubble",
            Self::Insertion => "/insertion",
            Self::Quick => "/quick",
            Self::Selection => "/selection",
        }
    }

    pub fn init(&self, params: SortParams) -> Box<dyn VisualSort> {
        match self {
            Self::Bubble => Box::new(bubble::Bubble::new(params)),
            Self::Insertion => Box::new(insertion::Insertion::new(params)),
            Self::Quick => Box::new(quick::Quick::new(params)),
            Self::Selection => Box::new(selection::Selection::new(params)),
        }
    }
}
