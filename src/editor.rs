use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode, KeyEvent, KeyModifiers,
};
use std::cmp::min;
use std::{env, io::Error};
mod terminal;
use terminal::{Position, Terminal};
mod view;
use view::View;

#[derive(Default)]
pub struct Editor {
    view: View,
    quit: bool,
    caret_location: Position,
}
impl Editor {
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        self.handle_args();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn handle_args(&mut self) {
        let args: Vec<String> = env::args().collect();
        if let Some(filename) = args.get(1) {
            self.view.load(String::from(filename)).unwrap();
        }
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

    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => self.quit = true,
                KeyCode::Char(c) => {
                    self.caret_location = self.view.write(c, self.caret_location.y)?;
                    Terminal::move_caret_to(self.caret_location)?;
                }
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageUp
                | KeyCode::PageDown
                | KeyCode::Home
                | KeyCode::End => {
                    self.move_point(*code)?;
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn move_point(&mut self, key_code: KeyCode) -> Result<(), Error> {
        let Position { mut x, mut y } = self.caret_location;
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
        self.caret_location = Position { x, y };
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), Error> {
        Terminal::hide_caret()?;
        if self.quit {
            Terminal::clear_screen()?;
            Terminal::print_at_position(Position::default(), "Exiting!\r\n")?;
        } else {
            self.view.render()?;
            Terminal::move_caret_to(self.caret_location)?;
        }
        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }
}
