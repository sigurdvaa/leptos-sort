pub mod bubble;
pub mod insertion;
pub mod quick;

pub use bubble::Bubble;
pub use insertion::Insertion;
pub use quick::Quick;

pub enum Routes {
    Bubble,
    Insertion,
    Quick,
}

impl Routes {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Bubble => "/bubble",
            Self::Insertion => "/insertion",
            Self::Quick => "/quick",
            // TODO: select sort
            // TODO: merge sort
        }
    }
}
