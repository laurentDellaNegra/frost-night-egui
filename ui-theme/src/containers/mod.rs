//! Generic Skyscope Design System containers.

mod accordion;
mod card;
mod drag_card;
mod surface;
mod tabs;

pub use accordion::{accordion, accordion_with_id};
pub use card::card;
pub use drag_card::{drag_card, DragCardResponse, DragCardState};
pub use surface::surface;
pub use tabs::{tabs, tabs_with_id};

#[cfg(feature = "icons")]
pub use tabs::{tabs_with_icons, tabs_with_icons_with_id};
