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

	// Math Functions
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
				_ => panic!("can only subtract ints"),
			},
			_ => panic!("can only subtract ints"),
		};
		self.push(Operator::Value(z));
	}

	pub fn multiply <'a> (&'a mut self) {
		let x = self.pop();
		let y = self.pop();
		let z = match x{
			Operator::Value(i) => match y{
				Operator::Value(j) => i*j,
				_ => panic!("can only multiply ints"),
			},
			_ => panic!("can only multiply ints"),
		};
		self.push(Operator::Value(z));
	}

	pub fn divide <'a> (&'a mut self) {
		let x = self.pop();
		let y = self.pop();
		let z = match x{
			Operator::Value(i) => match y{
				Operator::Value(j) => i/j,
				_ => panic!("can only divide ints"),
			},
			_ => panic!("can only divide ints"),
		};
		self.push(Operator::Value(z));
	}

	pub fn modulus <'a> (&'a mut self) {
		let x = self.pop();
		let y = self.pop();
		let z = match x{
			Operator::Value(i) => match y{
				Operator::Value(j) => i % j,
				_ => panic!("can only mod ints"),
			},
			_ => panic!("can only mod ints"),
		};
		self.push(Operator::Value(z));
	}

	pub fn and <'a> (&'a mut self) {
		let x = self.pop();
		let y = self.pop();
		let z = match x{
			Operator::Value(i) => match y{
				Operator::Value(j) => i & j,
				_ => panic!("can only and ints"),
			},
			_ => panic!("can only and ints"),
		};
		self.push(Operator::Value(z));
	}

	pub fn or <'a> (&'a mut self) {
		let x = self.pop();
		let y = self.pop();
		let z = match x{
			Operator::Value(i) => match y{
				Operator::Value(j) => i | j,
				_ => panic!("can only or ints"),
			},
			_ => panic!("can only or ints"),
		};
		self.push(Operator::Value(z));
	}

	pub fn xor <'a> (&'a mut self) {
		let x = self.pop();
		let y = self.pop();
		let z = match x{
			Operator::Value(i) => match y{
				Operator::Value(j) => i ^ j,
				_ => panic!("can only xor ints"),
			},
			_ => panic!("can only xor ints"),
		};
		self.push(Operator::Value(z));
	}

	pub fn lshift <'a> (&'a mut self) {
		let x = self.pop();
		let y = self.pop();
		let z = match x{
			Operator::Value(i) => match y{
				Operator::Value(j) => i << j,
				_ => panic!("can only lshift ints"),
			},
			_ => panic!("can only lshift ints"),
		};
		self.push(Operator::Value(z));
	}

	pub fn rshift <'a> (&'a mut self) {
		let x = self.pop();
		let y = self.pop();
		let z = match x{
			Operator::Value(i) => match y{
				Operator::Value(j) => i >> j,
				_ => panic!("can only rshift ints"),
			},
			_ => panic!("can only rshift ints"),
		};
		self.push(Operator::Value(z));
	}

	pub fn abs <'a> (&'a mut self) {
		let x = self.pop();
		let z = match x{
			Operator::Value(i) => if i < 0{-i} else {i},
			_ => panic!("can only abs an int"),
		};
		self.push(Operator::Value(z));
	}

	pub fn max <'a> (&'a mut self) {
		let x = self.pop();
		let y = self.pop();
		let z = match x{
			Operator::Value(i) => match y{
				Operator::Value(j) => if j > i {j} else{i},
				_ => panic!("can only max ints"),
			},
			_ => panic!("can only max ints"),
		};
		self.push(Operator::Value(z));
	}

	pub fn min <'a> (&'a mut self) {
		let x = self.pop();
		let y = self.pop();
		let z = match x{
			Operator::Value(i) => match y{
				Operator::Value(j) => if j < i {j} else {i},
				_ => panic!("can only min ints"),
			},
			_ => panic!("can only min ints"),
		};
		self.push(Operator::Value(z));
	}

	pub fn invert <'a> (&'a mut self) {
		let x = self.pop();
		let z = match x{
			Operator::Value(i) => !i,
			_ => panic!("can only invert an int"),
		};
		self.push(Operator::Value(z));
	}

	pub fn negate <'a> (&'a mut self) {
		let x = self.pop();
		let z = match x{
			Operator::Value(i) => -i,
			_ => panic!("can only negate an int"),
		};
		self.push(Operator::Value(z));
	}

	//Stack Functions
	pub fn dup <'a> (&'a mut self) {
		let x = self.pop();
		let y = x.clone();
		self.push(x);
		self.push(y);
	}

	pub fn drop <'a> (&'a mut self) {
		self.pop();
	}

	pub fn swap <'a> (&'a mut self) {
		let x = self.pop();
		let y = self.pop();
		self.push(x);
		self.push(y);
	}

	pub fn over <'a> (&'a mut self) {
		let x = self.pop();
		let y = self.pop();
		let z = x.clone();
		self.push(x);
		self.push(y);
		self.push(z);
	}

	pub fn rot <'a> (&'a mut self) {
		let x = self.pop();
		let y = self.pop();
		let z = self.pop();
		self.push(x);
		self.push(z);
		self.push(y);
	}

	//Debug Functions
	pub fn print <'a> (&'a mut self) {
		let x = self.pop();
		println!("{}",match x{
			Operator::Value(i) => i,
			_ => panic!("tried to print something other than a value"),
		});
	}
	
}

#[derive(Clone)]
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
	Modulus,
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
		"mod" => Operator::Modulus,
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

		if vm.stack.len() == 0{break;}
	}
}
