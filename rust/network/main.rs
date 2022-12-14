mod mes;
mod node;
use crate::node::{Node};
use std::{io::stdin, env, thread, sync::mpsc};

fn main() {
	let (main_tx, node_rx) = mpsc::channel();
	let mut node: Node = Node::new(node_rx);
	let args: Vec<String> = env::args().collect();
	if args.len() > 1 {
		let mut ip = args[1].clone();
		ip.push_str(":7878");
		node.add_ip(ip);
	}
	let hand = thread::spawn(move || {
		loop {
			let mut input = String::new();
			match stdin().read_line(&mut input) {
				Ok(_) => {
					match input.strip_suffix("\n") {
						Some(clean) => {
							match clean.chars().next() {
								Some(letter) => {
									match main_tx.send(clean.to_string()) {
										Ok(_) => {},
										Err(e) => {
											println!("cant send  input {e}");
										}
									}
									if letter as u8 == 27 {
										break;
									}
								},
								None => {}
							}
						},
						None => {}
					}
				},
				Err(_) => {}
			}
		}
	});
	node.run();
	match hand.join() {
		Ok(_) => {
		},
		Err(_) => {
			println!("error on input thread join ");
		}
	}
}
