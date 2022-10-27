
use std::io::{BufRead, BufReader};
use std::fs::File;
use crate::block::{
	Block,
	StatementBlock,
	ResourceBlock,
	IfBlock,
	CaseBlock,
	Property, UnknownBlock,
};

pub enum Status {
	Signature,
	Contents,
}

pub struct Parser<'a> {
	pub reader: BufReader<&'a std::fs::File>,
	pub block: Block,
	pub status: Status,
}

impl<'a> Parser<'a> {
	pub fn new(f: &File) -> Parser {
		let parser = Parser {
			reader: BufReader::new(f),
			block: Block::Eof,
			status: Status::Signature,
		};
		parser
	}

	pub fn next_line(&mut self) -> String {
		let mut line = String::new();

		loop {
			let num_bytes = self.reader.read_line(&mut line).expect("failed to read line");

			//ret EoF
			if num_bytes == 0 {
				return line
			}


			//strip Comment-out	
			for (i, l) in line.as_str().chars().enumerate() {
				if l == '#' {
					line = (&line[..i]).to_string();
					break
				}
			}

			//skip Blank-line, space, tab
			let white: &[_] = &[' ', '\t', '\n'];
			line = line.trim_start_matches(white).to_string();
			if line.len() == 0 { continue }

			return line
		}
	}
	
	// do_parse
	// BufからBlockを作る
	// Buffer(8KB)を読み切ったら次の8KBをBufに読む
	// とりあえず、Buf内にファイルが全て入るとして進める
	pub fn do_parse(mut self) -> Parser<'a> {

		let line = self.next_line();
		if line == "" { self.block = Block::Eof; return self } //EOF
		let words: Vec<&str> = line.as_str().split_whitespace().collect();

		// Is there "do" ?
		let mut do_flag: bool = false;
		let ws = words.to_vec();
		for word in ws {
			if word == "do" {
				do_flag = true;
				break;
			}
		}
		
		// Resource or ... ?
		if Parser::is_resource(&words[0]) {
			if !do_flag {
				self.block = ResourceBlock::create_def_block(words);
				self
			} else {
				ResourceBlock::create_block(words, self)
			}
		} else if do_flag {
			StatementBlock::create_block(words, self)
		} else if words[0] == "if" {
			let if_block: IfBlock = IfBlock::new();
			if_block.create_whole_if_block(&words, self)
		} else if words[0] == "case" {
			let case_block: CaseBlock = CaseBlock::new();
			case_block.create_whole_case_block(&words, self)
		} else if words[0] == "elsif" {
			self.block = Block::Elsif(vec![words[0].to_string(),words[1].to_string()]);
			self
		} else if words[0] == "else" {
			self.block = Block::Else;
			self
		} else if words[0] == "when" {
			self.block = Block::When(vec![words[0].to_string(),words[1].to_string()]);
			self
		} else if words[0] == "end" {
			self.status = Status::Signature;
			self.block = Block::End;
			self
		} else {
			if let Status::Contents = self.status {
				self.block = Property::create_property(words);
				self
			} else { 		// 読めない構文
				self.block = UnknownBlock::create_block(words);
				self
			}
		}
	}

	fn is_resource(word: &str) -> bool {
		let resources: Vec<&str> = vec![
			"alternatives", "apt_package", "apt_preference",
			"apt_repository", "apt_update", "archive_file",
			"bash", // 以下, 作為的に選択
			"file", "package", "template"
		];
		
		for resource_name in resources {
			if word == resource_name {
				return true
			} 
		}
		false
	}

}
