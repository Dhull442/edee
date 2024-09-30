
#![warn(clippy::all, clippy::pedantic, clippy::perf)]
mod editor;
use editor::Editor;

fn main() {
    Editor::default().run();
}
