pub mod bubble;
pub mod insert;
pub mod quick;

pub use bubble::Bubble;
pub use insert::Insert;
pub use quick::Quick;

pub enum Routes {
    Bubble,
    Insert,
    Quick,
}

impl Routes {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Bubble => "/bubble",
            Self::Insert => "/insert",
            Self::Quick => "/quick",
            // TODO: select sort
            // TODO: merge sort
        }
    }
}
