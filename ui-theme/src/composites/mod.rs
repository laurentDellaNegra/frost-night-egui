//! Optional higher-level Skyscope Design System compositions.
//!
//! These widgets combine generic components into opinionated tool surfaces.
//! They are gated behind the `composites` feature so host applications can
//! keep the core design-system dependency free of product-specific layouts.

#[cfg(feature = "icons")]
mod action_toolbar;
mod sidebar_card;
mod toolbar;
mod top_toolbar;
mod zoom_toolbar;

#[cfg(feature = "icons")]
pub use action_toolbar::{
    action_toolbar, action_toolbar_with_id, ActionToolbarItem, ActionToolbarResponse,
};
pub use sidebar_card::{sidebar_card, SidebarCardResponse};
pub use toolbar::{toolbar, toolbar_with_id, ToolbarGroup, ToolbarItem, ToolbarResponse};
pub use top_toolbar::{
    top_toolbar, top_toolbar_with_id, StatusField, StatusFieldKind, ToolbarAction,
    TopToolbarResponse,
};
pub use zoom_toolbar::{zoom_toolbar, zoom_toolbar_with_id, ZoomToolbarResponse};
