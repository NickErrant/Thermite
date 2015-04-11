use std::io;
use std::str::FromStr;

struct VM {
	stack: Vec<Operator>,
}

impl VM {
	fn pop <'a> (&'a mut self, ret: &mut Operator) {
		*ret = self.stack.pop().expect("gg reinstall");
	}

	fn push <'a> (&'a mut self, op: Operator) {
		self.stack.push(op);
	}
}

enum Operator {
	//stack
	Dup,
	Drop,
	Swap,
	Over,
	Rot,
	
	//debug
	Print,
	
	//math
	Add,
	Subtract,
	Multiply,
	Divide,
	Mod,
	And,
	Or,
	Xor,
	Lshift,
	Rshift,
	Abs,
	Max,
	Min,
	Invert,
	Negate,
	
	//custom
	Define,

	//values and custom calls
	Value(i32),
	Other,
}

fn parse(s: &str) -> Operator {
	match s {
		//stack operations:
		"dup" => Operator::Dup,
		"drop" => Operator::Drop,
		"swap" => Operator::Swap,
		"over" => Operator::Over,
		"rot" => Operator::Rot,

		//debug operations:
		"." => Operator::Print,
		
		//math operations:
		"+" => Operator::Add,
		"-" => Operator::Subtract,
		"*" => Operator::Multiply,
		"/" => Operator::Divide,
		"mod" => Operator::Mod,
		"and" => Operator::And,
		"or" => Operator::Or,
		"xor" => Operator::Xor,
		"lshift" => Operator::Lshift,
		"<<" => Operator::Lshift,
		"rshift" => Operator::Rshift,
		">>" => Operator::Rshift,
		"abs" => Operator::Abs,
		"max" => Operator::Max,
		"min" => Operator::Min,
		"invert" => Operator::Invert,
		"negate" => Operator::Negate,
		
		//custom definitions
		":" => Operator::Define,
		
		//value or custom operator
		x => {
			let y: Option<i32> = FromStr::from_str(x).ok();
			match y{
				Some(i) => Operator::Value(i),
				None => Operator::Other,
			}
		},
	}
}

fn process(s: String) {
	for o in s.split(" "){
		let b = parse(o.trim());
		println!("{}",match b {
			Operator::Add => "addition!",
			Operator::Subtract => "subtraction!",
			Operator::Value(_) => "value!",
			_ => "other!",
		});
	}
}

fn main() {
	
	let mut x = io::stdin();
	let mut vm = VM {stack: vec![Operator::Print]};

	loop{
		let mut y: String = "".to_string();
		let _ = x.read_line(&mut y);

	    process(y);
	}
}
