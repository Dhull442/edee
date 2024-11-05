use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::queue;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::io::stdout;

pub struct Terminal {}
pub struct Vector {
    pub width: u16,
    pub height: u16,
}

impl Vector {
    pub fn new((w, h): (u16, u16)) -> Vector {
        Vector {
            width: w,
            height: h,
        }
    }
}

impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen();
        Self::move_to(Vector::new((0, 0)))?;
        Ok(())
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode();
        Ok(())
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        queue!(stdout(), Clear(ClearType::All));
        Ok(())
    }

    pub fn move_to(v: Vector) -> Result<(), std::io::Error> {
        queue!(stdout(), MoveTo(v.width, v.height))?;
        Ok(())
    }

    pub fn size() -> Result<Vector, std::io::Error> {
        Ok(Vector::new(size()?))
    }

    pub fn hide_cursor() -> Result<(), std::io::Error> {
        Hide;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), std::io::Error> {
        Show;
        Ok(())
    }
}
