#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else
)]

#[path = "modules/rorlib.rs"]
mod rorlib;

#[path = "modules/terminal.rs"]
mod terminal;

#[path = "modules/document.rs"]
mod document;

#[path = "modules/row.rs"]
mod row;

use std::io::{self, stdout};
use rorlib::Editor;
pub use rorlib::SearchDirection;
pub use terminal::Terminal;
pub use rorlib::Position;
pub use document::Document;
pub use row::Row;

fn main() {
    Editor::default().run();
}
