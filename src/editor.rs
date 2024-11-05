use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io::Error;
mod terminal;
use terminal::{Terminal, Vector};

pub struct Editor {
    content: String,
    quit: bool,
    nlines: u16,
}

impl Editor {
    pub const fn default() -> Self {
        Self {
            content: String::new(),
            quit: false,
            nlines: 0,
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
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char(c) => {
                    if *modifiers == KeyModifiers::CONTROL {
                        match c {
                            'q' => self.quit = true,
                            _ => (),
                        }
                    } else {
                        self.content.push(*c);
                        self.nlines = 1;
                    }
                }
                // KeyCode(k) => {
                // 	if *k == KeyCode::RETURN {
                // 		self.nlines+=1;
                // 		self.content.push("\n");
                // 	}
                // }
                _ => (),
            }
        }
    }

    fn refresh_screen(&mut self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.quit {
            Terminal::clear_screen()?;
            Terminal::print("Exiting!\r\n")?;
        } else {
            Self::draw_rows()?;
            if self.content.len() < 1 {
                Terminal::print_welcome_message()?;
            }
            Terminal::move_cursor_to(Vector { x: 0, y: 0 })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Vector { y, .. } = Terminal::size()?;
        for current_row in 0..y {
            Terminal::move_cursor_to(Vector {
                x: 0,
                y: current_row,
            })?;
            Terminal::clear_line()?;
            Terminal::print("~")?;
        }
        Ok(())
    }
}
