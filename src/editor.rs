use crossterm::event::{read, Event::{self,Key}, KeyCode, KeyEvent, KeyModifiers};
use std::io::Error;
use std::cmp::min;
mod terminal;
use terminal::{Terminal, Vector};


#[derive(Default)]
pub struct Editor {
    content: String,
    quit: bool,
    caret_location: Vector,
}

impl Editor {
    pub const fn default() -> Self {
        Self {
            content: String::new(),
            caret_location: Vector{ x:0,y:0 },
            quit: false,
        }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.quit {
                break;
            }

            let event = read()?;
            self.evaluate_event(&event)?;
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(),Error> {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.quit = true
                }
                KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right | KeyCode::PageUp | KeyCode::PageDown | KeyCode::Home | KeyCode::End => {
                    self.move_point(*code)?;
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn move_point(&mut self, key_code: KeyCode) -> Result<(),Error> {
        let Vector{mut x, mut y} = self.caret_location;
        let size = Terminal::size()?;
        let h = size.x;
        let w = size.y;
        match key_code {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            }
            KeyCode::Down => {
                y = min(y.saturating_add(1), h.saturating_sub(1));
            }
            KeyCode::Right => {
                x = min(x.saturating_add(1), w.saturating_sub(1));
            }
            KeyCode::Left => {
                x = x.saturating_sub(1);
            }
            KeyCode::PageUp => {
                y = 0;
            }
            KeyCode::PageDown => {
                y = h.saturating_sub(1);
            }
            KeyCode::Home => {
                x = 0;
            }
            KeyCode::End => {
                x = w.saturating_sub(1);
            }
            _ => (),
        }
        self.caret_location = Vector{x,y};
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), Error> {
        Terminal::hide_caret()?;
        if self.quit {
            Terminal::clear_screen()?;
            Terminal::move_caret_to(Vector{x:0,y:0})?;
            Terminal::print("Exiting!\r\n")?;
        } else {
            Self::draw_rows()?;
            if self.content.len() < 1 {
                Terminal::print_welcome_message()?;
            }
            Terminal::move_caret_to(self.caret_location)?;
        }
        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Vector { y, .. } = Terminal::size()?;
        for current_row in 0..y {
            Terminal::move_caret_to(Vector {
                x: 0,
                y: current_row,
            })?;
            Terminal::clear_line()?;
            Terminal::print("~")?;
        }
        Ok(())
    }
}
