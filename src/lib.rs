#![warn(clippy::pedantic, clippy::nursery, clippy::cargo, unused)]

mod prompt;
mod status;

mod text_prompt;
mod text_state;

pub use prompt::*;
pub use status::*;

pub use text_prompt::*;
pub use text_state::*;

pub mod prelude {
    pub use crate::Focus;
    pub use crate::Prompt;
    pub use crate::State;
    pub use crate::Status;
    pub use crate::TextPrompt;
    pub use crate::TextRenderStyle;
    pub use crate::TextState;
}
