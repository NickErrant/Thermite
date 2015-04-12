use std::io;
use std::str::FromStr;
use std::collections::HashMap;

pub struct VM {
	op_stack: Vec<Operator>,
	data_stack: Vec<Operator>,
	defs: HashMap<String, Vec<Operator>>,
}

impl VM {
	pub fn pop_data <'a> (&'a mut self) -> Operator {
		return self.data_stack.pop().expect("data stack is empty!");
	}

	pub fn push_data <'a> (&'a mut self, op: Operator) {
		self.data_stack.push(op);
	}

	pub fn pop_op <'a> (&'a mut self) -> Operator {
		return self.op_stack.pop().expect("operand stack is empty!");
	}

	pub fn push_op <'a> (&'a mut self, op: Operator) {
		self.op_stack.push(op);
	}

	pub fn eval <'a> (&'a mut self) {
		let op = self.pop_op();
		match op {
			Operator::Add => self.add(),
			Operator::Subtract => self.subtract(),
			Operator::Multiply => self.multiply(),
			Operator::Divide => self.divide(),
			Operator::Modulus => self.modulus(),
			Operator::And => self.and(),
			Operator::Or => self.or(),
			Operator::Xor => self.xor(),
			Operator::Lshift => self.lshift(),
			Operator::Rshift => self.rshift(),
			Operator::Abs => self.abs(),
			Operator::Max => self.max(),
			Operator::Min => self.min(),
			Operator::Invert => self.invert(),
			Operator::Negate => self.negate(),
			Operator::Dup => self.dup(),
			Operator::Drop => self.drop(),
			Operator::Swap => self.swap(),
			Operator::Over => self.over(),
			Operator::Rot => self.rot(),
			Operator::Value(i) => self.push_data(Operator::Value(i)),
			Operator::Print => self.print(),
			_ => panic!("soon")
		}
	}

	// Math Functions
	pub fn add <'a> (&'a mut self) {
		let x = self.pop_data();
		let y = self.pop_data();
		let z = match x{
			Operator::Value(i) => match y{
				Operator::Value(j) => i+j,
				_ => panic!("can only add ints"),
			},
			_ => panic!("can only add ints"),
		};
		self.push_data(Operator::Value(z));
	}

	pub fn subtract <'a> (&'a mut self) {
		let x = self.pop_data();
		let y = self.pop_data();
		let z = match x{
			Operator::Value(i) => match y{
				Operator::Value(j) => i-j,
				_ => panic!("can only subtract ints"),
			},
			_ => panic!("can only subtract ints"),
		};
		self.push_data(Operator::Value(z));
	}

	pub fn multiply <'a> (&'a mut self) {
		let x = self.pop_data();
		let y = self.pop_data();
		let z = match x{
			Operator::Value(i) => match y{
				Operator::Value(j) => i*j,
				_ => panic!("can only multiply ints"),
			},
			_ => panic!("can only multiply ints"),
		};
		self.push_data(Operator::Value(z));
	}

	pub fn divide <'a> (&'a mut self) {
		let x = self.pop_data();
		let y = self.pop_data();
		let z = match x{
			Operator::Value(i) => match y{
				Operator::Value(j) => i/j,
				_ => panic!("can only divide ints"),
			},
			_ => panic!("can only divide ints"),
		};
		self.push_data(Operator::Value(z));
	}

	pub fn modulus <'a> (&'a mut self) {
		let x = self.pop_data();
		let y = self.pop_data();
		let z = match x{
			Operator::Value(i) => match y{
				Operator::Value(j) => i % j,
				_ => panic!("can only mod ints"),
			},
			_ => panic!("can only mod ints"),
		};
		self.push_data(Operator::Value(z));
	}

	pub fn and <'a> (&'a mut self) {
		let x = self.pop_data();
		let y = self.pop_data();
		let z = match x{
			Operator::Value(i) => match y{
				Operator::Value(j) => i & j,
				_ => panic!("can only and ints"),
			},
			_ => panic!("can only and ints"),
		};
		self.push_data(Operator::Value(z));
	}

	pub fn or <'a> (&'a mut self) {
		let x = self.pop_data();
		let y = self.pop_data();
		let z = match x{
			Operator::Value(i) => match y{
				Operator::Value(j) => i | j,
				_ => panic!("can only or ints"),
			},
			_ => panic!("can only or ints"),
		};
		self.push_data(Operator::Value(z));
	}

	pub fn xor <'a> (&'a mut self) {
		let x = self.pop_data();
		let y = self.pop_data();
		let z = match x{
			Operator::Value(i) => match y{
				Operator::Value(j) => i ^ j,
				_ => panic!("can only xor ints"),
			},
			_ => panic!("can only xor ints"),
		};
		self.push_data(Operator::Value(z));
	}

	pub fn lshift <'a> (&'a mut self) {
		let x = self.pop_data();
		let y = self.pop_data();
		let z = match x{
			Operator::Value(i) => match y{
				Operator::Value(j) => i << j,
				_ => panic!("can only lshift ints"),
			},
			_ => panic!("can only lshift ints"),
		};
		self.push_data(Operator::Value(z));
	}

	pub fn rshift <'a> (&'a mut self) {
		let x = self.pop_data();
		let y = self.pop_data();
		let z = match x{
			Operator::Value(i) => match y{
				Operator::Value(j) => i >> j,
				_ => panic!("can only rshift ints"),
			},
			_ => panic!("can only rshift ints"),
		};
		self.push_data(Operator::Value(z));
	}

	pub fn abs <'a> (&'a mut self) {
		let x = self.pop_data();
		let z = match x{
			Operator::Value(i) => if i < 0{-i} else {i},
			_ => panic!("can only abs an int"),
		};
		self.push_data(Operator::Value(z));
	}

	pub fn max <'a> (&'a mut self) {
		let x = self.pop_data();
		let y = self.pop_data();
		let z = match x{
			Operator::Value(i) => match y{
				Operator::Value(j) => if j > i {j} else{i},
				_ => panic!("can only max ints"),
			},
			_ => panic!("can only max ints"),
		};
		self.push_data(Operator::Value(z));
	}

	pub fn min <'a> (&'a mut self) {
		let x = self.pop_data();
		let y = self.pop_data();
		let z = match x{
			Operator::Value(i) => match y{
				Operator::Value(j) => if j < i {j} else {i},
				_ => panic!("can only min ints"),
			},
			_ => panic!("can only min ints"),
		};
		self.push_data(Operator::Value(z));
	}

	pub fn invert <'a> (&'a mut self) {
		let x = self.pop_data();
		let z = match x{
			Operator::Value(i) => !i,
			_ => panic!("can only invert an int"),
		};
		self.push_data(Operator::Value(z));
	}

	pub fn negate <'a> (&'a mut self) {
		let x = self.pop_data();
		let z = match x{
			Operator::Value(i) => -i,
			_ => panic!("can only negate an int"),
		};
		self.push_data(Operator::Value(z));
	}

	//Stack Functions
	pub fn dup <'a> (&'a mut self) {
		let x = self.pop_data();
		let y = x.clone();
		self.push_data(x);
		self.push_data(y);
	}

	pub fn drop <'a> (&'a mut self) {
		self.pop_data();
	}

	pub fn swap <'a> (&'a mut self) {
		let x = self.pop_data();
		let y = self.pop_data();
		self.push_data(x);
		self.push_data(y);
	}

	pub fn over <'a> (&'a mut self) {
		let x = self.pop_data();
		let y = self.pop_data();
		let z = x.clone();
		self.push_data(x);
		self.push_data(y);
		self.push_data(z);
	}

	pub fn rot <'a> (&'a mut self) {
		let x = self.pop_data();
		let y = self.pop_data();
		let z = self.pop_data();
		self.push_data(x);
		self.push_data(z);
		self.push_data(y);
	}

	//Debug Functions
	pub fn print <'a> (&'a mut self) {
		let x = self.pop_data();
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

impl Operator {
	fn stringify(&self) -> String {
		match *self{
			Operator::Add => format!("+"),
			Operator::Subtract => format!("-"),
			Operator::Multiply => format!("*"),
			Operator::Divide => format!("/"),
			Operator::Modulus => format!("mod"),
			Operator::And => format!("and"),
			Operator::Or => format!("or"),
			Operator::Xor => format!("xor"),
			Operator::Lshift => format!("<<"),
			Operator::Rshift => format!(">>"),
			Operator::Abs => format!("abs"),
			Operator::Max => format!("max"),
			Operator::Min => format!("min"),
			Operator::Invert => format!("invert"),
			Operator::Negate => format!("negate"),
			Operator::Dup => format!("dup"),
			Operator::Drop => format!("drop"),
			Operator::Swap => format!("swap"),
			Operator::Over => format!("over"),
			Operator::Rot => format!("rot"),
			Operator::Value(i) => format!("{}", i),
			Operator::Print => format!("."),
			Operator::Define => format!(":"),
			Operator::Other => format!("other"),
		}
	}
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
	let mut vm = VM {op_stack: vec![],
					 data_stack: vec![],
					 defs: HashMap::new()};

	loop{
		let mut line: String = "".to_string();
		let _ = x.read_line(&mut line);

		if line.trim() == "quit" {break;}

		for o in line.split(" "){
			let b = parse(o.trim());
			vm.push_op(b);
		}

		while vm.op_stack.len() > 0 {
			vm.eval();
		}
	}
}
