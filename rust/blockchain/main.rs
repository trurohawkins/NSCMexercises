mod blockchain;
use crate::blockchain::{Block, Chain, choose_chain};

fn main() {
	let mut chain: Chain<String> = Chain::new();
	let data: Vec<String> = vec!["eggs".to_string(), "dunk".to_string(), "eat".to_string()];
	let b1: Block<String> = Block::new(chain.blocks.last().expect("there is at least the genesis block"), data);
	if chain.try_add_block(b1) {
		println!("block added succesfully");
	} else {
		println!("bad block");
	}
	let data2: Vec<String> = vec!["sexy".to_string(), "dad".to_string()];
	let b2: Block<String> = Block::new(&chain.blocks[0], data2);
	if chain.try_add_block(b2) {
		println!("block added succesfully");
	} else {
		println!("bad block");
	}

	if chain.is_valid() {
		println!("valid chain");
	}	else {
		println!("invalid chain");
	}

	println!("\nNEW CHAIN\n");
	let mut chain2: Chain<String> = Chain::new();
	let data3: Vec<String> = vec!["forpo".to_string(), "klij".to_string()];
	let data4: Vec<String> = vec!["pingpong".to_string(), "meso".to_string()];
	let data5: Vec<String> = vec!["gelding".to_string(), "terror two".to_string()];
	let b3: Block<String> = Block::new(chain2.blocks.last().expect("at least the genesis block"), data3);
	if chain2.try_add_block(b3) {
		println!("block added succesfully");
	} else {
		println!("bad block");
	}
	let b4: Block<String> = Block::new(chain2.blocks.last().expect("at lest the genesis block"), data4);
	if chain2.try_add_block(b4) {
		println!("block added succesfully");
	} else {
		println!("bad block");
	}

	println!("choosing between ccurrent {:#?} and new {:#?}", chain, chain2);
	let chosen = choose_chain(&chain, &chain2);
	if *chosen == chain2 {
		println!("chose new chain");
		chain = chosen.clone();
	} else {
		println!("chain is good");
	}

	println!("{:#?}", chain);
	let b5: Block<String> = Block::new(chain.blocks.last().expect("at lest the genesis block"), data5);
	if chain.try_add_block(b5) {
		println!("block added succesfully");
	} else {
		println!("bad block");
	}
	let chosen = choose_chain(&chain, &chain2);
	if *chosen == chain2 {
		println!("chose new chain");
		chain = chosen.clone();
	} else {
		println!("we are the longest");
	}

	println!("{:#?}", chain);
	chain.save_chain();
	chain = Chain::load_chain();
	println!("{:#?}", chain);
}
