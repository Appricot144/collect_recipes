use serde::Serialize;
use crate::parser::Parser;

#[derive(Debug, Clone, Serialize)]
pub enum Block {
	Statement(StatementBlock),
	Resource(ResourceBlock),
	Property(Property),
	Case(CaseBlock),
	If(IfBlock),
	When(Vec<String>),	// 
	Elsif(Vec<String>),	// この辺のBlockは制御用のもの
	Else,				// 構造体のcontentsには、このBlockの種類は格納されない
	End,				//
	Eof,		
}

// enum WordType {	// トークンを定義するならこんな感じだろう
// 	ChefKeyword,
// 	String,
// 	Symbol,
// 	Do,
// 	End,
// 	Statement,
// 	Other,
// }

// impl Block {
// 	pub fn iter(&self) {
// 	}
// 	
// 	pub fn print_block(&self) {
// 	}
// }

#[derive(Debug, Clone, Serialize)]
pub struct StatementBlock {
	statement_type: String,
	statement_name: String,
	contents: Vec<Block>,
}

impl StatementBlock {
	pub fn create_block<'a>(words: Vec<&str>, mut parser: Parser<'a>) -> Parser<'a> {
		let statement_block = StatementBlock {
			statement_type: words[0].to_string(),
			statement_name: words[1].to_string(),
			contents: {
				let mut blocks: Vec<Block> = Vec::new();
				loop {
					parser = parser.do_parse();
					if let Block::End = parser.block {
						break
					}
					blocks.push(parser.block.clone());
				}
				blocks
			},		
		};
		parser.block = Block::Statement(statement_block);
		parser
	}
}

#[derive(Debug, Clone, Serialize)]
enum IfStatus {
	If,
	Elsif(Vec<String>),
	Else,
	End,
}

#[derive(Debug, Clone, Serialize)]
pub struct IfBlock {
	blocks: Vec<Block>,
	status: IfStatus,
}

impl IfBlock {
	pub fn new() -> Self {
		IfBlock { blocks: Vec::new(), status: IfStatus::If }
	}
	
	pub fn create_whole_if_block<'a>(mut self, words: &[&str], mut parser: Parser<'a>) -> Parser<'a>{
		self.status = IfStatus::If;
		parser = self.create_if_block(&words, parser);

		loop {
			match self.status {
				IfStatus::Elsif(ref exp) => {
					parser = self.create_elsif_block(exp.to_vec(), parser);
				},	
				IfStatus::Else => {
					parser = self.create_else_block(parser);
				},
				IfStatus::End =>{
					break
				},
				_ => panic!(),
			}
		}
		parser.block = Block::If(self);
		parser
	}

	// if文部分のみをIfBlockに格納(push)する
	fn create_if_block<'a>(&mut self, words: &[&str], mut parser: Parser<'a>) -> Parser<'a> {
		let if_blk = StatementBlock {
			statement_type: words[0].to_string(),
			statement_name: words[1].to_string(),
			contents: {
				let mut blocks: Vec<Block> = Vec::new();
				loop {
					parser = parser.do_parse();
					match parser.block {
						Block::End => {
							self.status = IfStatus::End;
							break
						},
						Block::Elsif(ref exp) => {
							self.status = IfStatus::Elsif(exp.clone());
							break
						},
						Block::Else => {
							self.status = IfStatus::Else;
							break
						},
						_ => {
							blocks.push(parser.block.clone());
						}
					}
				}
				blocks
			},
		};
		self.blocks.push(Block::Statement(if_blk)); // if 文の格納
		parser
	}
	
	fn create_elsif_block<'a>(&mut self, words: Vec<String>, mut parser: Parser<'a>) -> Parser<'a>{
		let elsif_blk = StatementBlock {
			statement_type: words[0].clone(),
			statement_name: words[1].clone(),
			contents: {
				let mut blocks: Vec<Block> = Vec::new();
				loop {
					parser = parser.do_parse();
					match parser.block {
						Block::End => {
							self.status = IfStatus::End;
							break
						},
						Block::Elsif(ref exp) => {
							self.status = IfStatus::Elsif(exp.clone());
							break
						},
						Block::Else => {
							self.status = IfStatus::Else;
							break
						},
						_ => {
							blocks.push(parser.block.clone());
						}
					}
				}
				blocks
			},
		};
		self.blocks.push(Block::Statement(elsif_blk)); // elsif 文の格納
		parser
	}
	
	fn create_else_block<'a>(&mut self, mut parser: Parser<'a>) -> Parser<'a>{
		let else_blk = StatementBlock {
			statement_type: "else".to_string(),
			statement_name: "".to_string(),
			contents: {
				let mut blocks: Vec<Block> = Vec::new();
				loop {
					parser = parser.do_parse();
					match parser.block {
						Block::End => {
							self.status = IfStatus::End;
							break
						},
						Block::Elsif(_exp) => {
							panic!()
						},
						Block::Else => {
							panic!()
						},
						_ => {
							blocks.push(parser.block.clone());
						}
					}
				}
				blocks
			},
		};
		self.blocks.push(Block::Statement(else_blk)); // else 文の格納
		parser
	}
}

#[derive(Debug, Clone, Serialize)]
enum CaseStatus {
	Case,
	When(Vec<String>),
	Else,
	End,
}

#[derive(Debug, Clone, Serialize)]
pub struct CaseBlock {
	blocks: Vec<Block>,
	status: CaseStatus,
}

impl CaseBlock {
	pub fn new() -> Self {
		CaseBlock { blocks: Vec::new(), status: CaseStatus::Case }
	}
	
	pub fn create_whole_case_block<'a>(mut self, words: &[&str], mut parser: Parser<'a>) -> Parser<'a> {
		self.status = CaseStatus::Case;
		parser = self.create_case_block(&words, parser);

		loop {
			match self.status {
				CaseStatus::When(ref exp) => {
					parser = self.create_when_block(exp.to_vec(), parser);
				},
				CaseStatus::Else => {
					parser = self.create_else_block(parser);
					break
				},
				CaseStatus::End =>{
					break
				},
				_ => panic!(),
			}
		}
		parser.block = Block::Case(self);
		parser
	}
	
	fn create_case_block<'a>(&mut self, words: &[&str], mut parser: Parser<'a>) -> Parser<'a> {
		let case_blk = StatementBlock {
			statement_type: words[0].to_string(),
			statement_name: words[1].to_string(),
			contents: {
				let mut blocks: Vec<Block> = Vec::new();
				loop {
					parser = parser.do_parse();
					match parser.block {
						Block::End => {
							self.status = CaseStatus::End;
							break
						},
						Block::When(ref exp) => {
							self.status = CaseStatus::When(exp.clone());
							break
						},
						Block::Else => {
							self.status = CaseStatus::Else;
							break
						},
						_ => {
							blocks.push(parser.block.clone());
						}
					}
				}
				blocks
			},
		};
		self.blocks.push(Block::Statement(case_blk)); // case 文の格納
		parser
	}
	
	fn create_when_block<'a>(&mut self, words: Vec<String>, mut parser: Parser<'a>) -> Parser<'a> {
		let when_blk = StatementBlock {
			statement_type: words[0].clone(),
			statement_name: words[1].clone(),
			contents: {
				let mut blocks: Vec<Block> = Vec::new();
				loop {
					parser = parser.do_parse();
					match parser.block {
						Block::End => {
							self.status = CaseStatus::End;
							break
						},
						Block::When(ref exp) => {
							self.status = CaseStatus::When(exp.clone());
							break
						},
						Block::Else => {
							self.status = CaseStatus::Else;
							break
						},
						_ => {
							blocks.push(parser.block.clone());
						}
					}
				}
				blocks
			},
		};
		self.blocks.push(Block::Statement(when_blk)); // elsif 文の格納
		parser
	}
	
	fn create_else_block<'a>(&mut self, mut parser: Parser<'a>) -> Parser<'a> {
		let else_blk = StatementBlock {
			statement_type: "else".to_string(),
			statement_name: "".to_string(),
			contents: {
				let mut blocks: Vec<Block> = Vec::new();
				loop {
					parser = parser.do_parse();
					match parser.block {
						Block::End => {
							self.status = CaseStatus::End;
							break
						},
						Block::When(_exp) => {
							panic!()
						},
						Block::Else => {
							panic!()
						},
						_ => {
							blocks.push(parser.block.clone());
						}
					}
				}
				blocks
			},
		};
		self.blocks.push(Block::Statement(else_blk)); // case 文の格納
		parser
	}	
}

#[derive(Debug, Clone, Serialize)]
pub struct ResourceBlock {
	resource_type: String,
	resource_name: String,
	contents: Vec<Block>,
}

impl ResourceBlock {
	pub fn create_def_block(words: Vec<&str>) -> Block {
		let resource_block = ResourceBlock {
			resource_type: words[0].to_string(),
			resource_name: words[1].to_string(),
			contents: { Vec::new() },
		};
		Block::Resource(resource_block)
	}
	
	pub fn create_block<'a>(words: Vec<&str>, mut parser: Parser<'a>) -> Parser<'a> {
		let resource_block = ResourceBlock {
			resource_type: words[0].to_string(),
			resource_name: words[1].to_string(),
			contents: {
				let mut blocks: Vec<Block> = Vec::new();
				loop {
					parser = parser.do_parse();
					if let Block::End = parser.block {
						break
					}
					blocks.push(parser.block.clone());
				}
				blocks
			},
		};
		
		parser.block = Block::Resource(resource_block);
		parser
	}
}

#[derive(Debug, Clone, Serialize)]
pub struct Property {
	property: String,
	value: String,
}

impl Property {
	pub fn create_property(words: Vec<&str>) -> Block {
		let property = Property {
			property: words[0].to_string(),
			value: words[1].to_string(),
		};
		Block::Property(property)
	}
}
