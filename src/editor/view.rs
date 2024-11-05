use super::terminal::{Position, Terminal};
use std::cmp::min;
use std::io::Error;

mod buffer;
use buffer::Buffer;
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct View {
    buffer: Buffer,
}
impl View {
    // pub const fn default() -> Self {
    //     Self{
    //         buffer: Buffer::default(),
    //     }
    // }
    pub fn render(&mut self) -> Result<(), Error> {
        let Position { y, .. } = Terminal::size()?;
        for current_row in 0..y {
            Terminal::move_caret_to(Position {
                x: 0,
                y: current_row,
            })?;
            Terminal::clear_line()?;
            if let Some(line) = self.buffer.lines.get(current_row) {
                Terminal::print(line)?;
                Terminal::print("\r\n")?;
                continue;
            }
            Terminal::print("~")?;
            if current_row.saturating_add(1) < y {
                Terminal::print("\r\n")?;
            }
        }
        if self.buffer.is_empty() {
            Self::render_welcome_message()?;
        }
        Ok(())
    }
    pub fn render_welcome_message() -> Result<(), Error> {
        let size = Terminal::size()?;
        let mut line = format!("{NAME}: a friendly text-editor in Rust\r");
        let mut width = min(line.len(), size.x);
        Terminal::print_at_position(
            Position {
                x: (size.x - width) / 2,
                y: size.y / 3 - 1,
            },
            &line,
        )?;
        line = format!("v{VERSION}\r");
        width = min(line.len(), size.x);
        Terminal::print_at_position(
            Position {
                x: (size.x - width) / 2,
                y: size.y / 3,
            },
            &line,
        )?;
        line = format!("by Saksham Dhull\r");
        width = min(line.len(), size.x);
        Terminal::print_at_position(
            Position {
                x: (size.x - width) / 2,
                y: size.y / 3 + 1,
            },
            &line,
        )?;
        Ok(())
    }
    pub fn write(&mut self, chr: &char, row: usize) -> Result<Position, Error> {
        while row >= self.buffer.lines.len() {
            self.buffer.lines.push(String::new());
        }
        self.buffer.lines[row].push(*chr);
        Ok(Position {
            x: self.buffer.lines[row].len(),
            y: row,
        })
    }

    pub fn load(&mut self, filename: String) -> Result<(), Error> {
        self.buffer = Buffer::load(filename)?;
        Ok(())
    }
}
