use std::{thread, time, io::{stdin, Write}, sync::mpsc};
use std::thread::sleep;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};

fn main() {
	let (tx, rx) = mpsc::channel();
	enable_raw_mode();
	let handle = thread::spawn(move || {
	loop {
		println!("loop iteration");
		match read().unwrap() {
			Event::Key(KeyEvent {
				code: KeyCode::Char('q'),..
			}) => {
				tx.send("quit".to_string());
				break;
			},
			Event::Key(KeyEvent {
				code: KeyCode::Char('h'),..
			}) => {
				tx.send("hello".to_string());
			},
			_ => {}
		}

	}
	});
	let millis = time::Duration::from_millis(1000);
	loop {
		match rx.try_recv() {
			Ok(r) => {
				if r == "quit".to_string() {
					break;
				}
				println!("receivefd {}", r);
			},
			Err(_) => {}
		}	
		println!("main loop");
		sleep(millis);

	}
	handle.join().unwrap();
	disable_raw_mode();
}

