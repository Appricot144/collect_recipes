// chef recipe を受け取り、コードの構造データをmongoDBに登録する
mod block;	
mod parser;	
mod mongo;	

use std::env;
use std::fs::File;
// use futures::stream::TryStreamExt;
use crate::block::Block;
use crate::parser::Parser;
use crate::mongo::ConnectionInfo;


fn parse_file(f: &File) -> Vec<Block>{
	let mut blocks: Vec<Block> = Vec::new();
	let mut parser = Parser::new(f);
	loop {
		parser = parser.do_parse();
		if let Block::Eof = parser.block { break }
		blocks.push(parser.block.clone());
	}
	blocks
}

fn main() {
	// args
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

	// open file
	println!("In recipe file {}",filename);
	let f = File::open(filename).expect("Failed open file");

	// parse recipe file
	println!("parsing recipe file");
	let blocks = parse_file(&f);

	// print blocks
	// for block in blocks.to_vec() {
	// 	block.print_block();
	// }

	// connecting to mongo
	println!("connecting to my mongoDB");
	println!("inserting documents into my collection");
	let mongo = ConnectionInfo::new();
	let ls = tokio::task::LocalSet::new();
	let rt = tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.build()
		.unwrap();
	ls.block_on(&rt, mongo.insert_structual_data(blocks));
}
