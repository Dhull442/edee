use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use std::io::stdout;
use std::io::Write;
mod terminal;
use terminal::Terminal;
use terminal::Vector;

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

    fn repl(&mut self) -> Result<(), std::io::Error> {
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
                        self.quit = true;
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

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        Terminal::clear_screen()?;
        Terminal::move_to(Vector::new((0, 0)));
        // Print("{}\r",self.content);
        // stdout.flush();
        Self::draw_rows()?;
        if self.quit {
            Print("Exiting!\r\n");
            stdout().flush();
        }
        Terminal::show_cursor()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        let v: Vector = Terminal::size()?;
        for i in 1..v.height {
            Print("~");
            if i < v.height - 1 {
                Print("\r\n");
            }
        }
        stdout().flush();
        Ok(())
    }
}
