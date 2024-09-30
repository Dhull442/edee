use crossterm::terminal::{disable_raw_mode, enable_raw_mode,Clear, ClearType};
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::execute;
use std::io::stdout;


pub struct Editor {
	content: String,
	quit: bool,
}

impl Editor{
	pub fn default() -> Editor {
		Editor{content: String::new(),quit:false}
	}

	pub fn run(&mut self) {
		Self::initialize().unwrap();
		let result = self.repl();
		Self::terminate().unwrap();
		result.unwrap();
	}

	fn initialize() -> Result<(),std::io::Error> {
		enable_raw_mode()?;
		Self::clear_screen()
	}

	fn terminate() -> Result<(), std::io::Error> {
		disable_raw_mode()
	}

	fn clear_screen() -> Result<(),std::io::Error> {
		let mut stdout = stdout();
		execute!(stdout, Clear(ClearType::All))
	}
	fn repl(&mut self) -> Result<(),std::io::Error> {
	    loop {
	    	let event = read()?;
	    	self.evaluate_event(&event);
	    	self.refresh_screen()?;
	    	if self.quit {break;}
	    }
	    Ok(())
	}
	fn evaluate_event(&mut self, event: &Event){
		if let Key(KeyEvent{
			code, modifiers, ..
		}) = event {
			match code {
				Char('q') if *modifiers == KeyModifiers::CONTROL=> {
					self.quit = true;
				}
				_ =>() ,
			}
		}
	}
	fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
		if self.quit {
			Self::clear_screen()?;
			print!("Exiting!\r\n");
		} else {
			println!("{}",self.content);	
		}
		Ok(())
	}
}