use std::io;
use std::str::FromStr;

pub struct VM {
	stack: Vec<Operator>,
}

impl VM {
	pub fn pop <'a> (&'a mut self) -> Operator {
		return self.stack.pop().expect("gg reinstall");
	}

	pub fn push <'a> (&'a mut self, op: Operator) {
		self.stack.push(op);
	}

	pub fn eval <'a> (&'a mut self, op: Operator) {
		
	}

	pub fn add <'a> (&'a mut self) {
		let x = self.pop();
		let y = self.pop();
		let z = match x{
			Operator::Value(i) => match y{
				Operator::Value(j) => i+j,
				_ => panic!("can only add ints"),
			},
			_ => panic!("can only add ints"),
		};
		self.push(Operator::Value(z));
	}

	pub fn subtract <'a> (&'a mut self) {
		let x = self.pop();
		let y = self.pop();
		let z = match x{
			Operator::Value(i) => match y{
				Operator::Value(j) => i-j,
				_ => panic!("can only add ints"),
			},
			_ => panic!("can only add ints"),
		};
		self.push(Operator::Value(z));
	}

	pub fn multiply <'a> (&'a mut self) {
		let x = self.pop();
		let y = self.pop();
		let z = match x{
			Operator::Value(i) => match y{
				Operator::Value(j) => i*j,
				_ => panic!("can only add ints"),
			},
			_ => panic!("can only add ints"),
		};
		self.push(Operator::Value(z));
	}

	pub fn divide <'a> (&'a mut self) {
		let x = self.pop();
		let y = self.pop();
		let z = match x{
			Operator::Value(i) => match y{
				Operator::Value(j) => i/j,
				_ => panic!("can only add ints"),
			},
			_ => panic!("can only add ints"),
		};
		self.push(Operator::Value(z));
	}
}

pub enum Operator {
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

fn main() {
	let mut x = io::stdin();
	let mut vm = VM {stack: vec![Operator::Print]};

	loop{
		let mut line: String = "".to_string();
		let _ = x.read_line(&mut line);

		for o in line.split(" "){
			println!("evaling: {}", o.trim());
			let b = parse(o.trim());
			vm.eval(b);
		}
	}
}
