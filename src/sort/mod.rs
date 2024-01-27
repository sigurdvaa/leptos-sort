pub mod bubblesort;
pub mod quicksort;

pub use bubblesort::Bubble;
pub use quicksort::Quick;

pub enum Routes {
    Bubble,
    Quick,
}

impl Routes {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Bubble => "/bubblesort",
            Self::Quick => "/quicksort",
        }
    }
}
