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


fn parse_file(f: &File, filename: String) -> Vec<Block>{
	let mut blocks: Vec<Block> = Vec::new();
	let mut parser = Parser::new(f, filename);
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
	println!(" -- In recipe file \"{}\"",filename);
	let f = File::open(filename).expect("Failed open file");

	// parse recipe file
	let blocks = parse_file(&f, filename.to_string());

	// print blocks
	// for block in blocks.to_vec() {
	// 	block.print_block();
	// }

	// no documents
	if blocks.len() == 0 {
		panic!("no-document found");
	}

	// connect mongo
	let mongo = ConnectionInfo::new();
	let ls = tokio::task::LocalSet::new();
	let rt = tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.build()
		.unwrap();
	ls.block_on(&rt, mongo.insert_structual_data(blocks));
	println!("[\x1b[{}mOk\x1b[m] Documents inserted", 32);
}
