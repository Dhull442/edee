use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::{execute, queue, Command};
use std::io::{stdout, Error, Write};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Copy, Clone)]
pub struct Vector {
    pub x: u16,
    pub y: u16,
}

pub struct Terminal;
impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::print_welcome_message()?;
        Self::move_cursor_to(Vector { x: 0, y: 0 })?;
        Self::execute()?;
        Ok(())
    }

    pub fn print_welcome_message() -> Result<(), Error> {
        let size = Self::size()?;
        let line0 = format!("{NAME}: a friendly text-editor in Rust\r");
        let half_width0: u16 = (line0.len() as u16) / 2;
        let line1 = format!("v{VERSION}\r");
        let half_width1: u16 = (line1.len() as u16) / 2;
        let line2 = format!("by Saksham Dhull\r");
        let half_width2: u16 = (line2.len() as u16) / 2;
        Self::print_at_position(
            Vector {
                x: size.x / 2 - half_width0,
                y: size.y / 3 - 1,
            },
            &line0,
        )?;
        Self::print_at_position(
            Vector {
                x: size.x / 2 - half_width1,
                y: size.y / 3,
            },
            &line1,
        )?;
        Self::print_at_position(
            Vector {
                x: size.x / 2 - half_width2,
                y: size.y / 3 + 1,
            },
            &line2,
        )?;
        Ok(())
    }

    pub fn print_at_position(position: Vector, string: &str) -> Result<(), Error> {
        Self::move_cursor_to(position)?;
        Self::print(string)?;
        Ok(())
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn move_cursor_to(position: Vector) -> Result<(), Error> {
        Self::queue_command(MoveTo(position.x, position.y))?;
        Ok(())
    }

    pub fn size() -> Result<Vector, Error> {
        let (width, height) = size()?;
        Ok(Vector {
            x: width,
            y: height,
        })
    }

    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }

    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
}
