use std::io;
use std::str::FromStr;
use std::collections::HashMap;


enum VMState {
	Definition,
	Execution
}

pub struct VM {
	state: VMState,
	op_stack: Vec<Operator>,
	data_stack: Vec<Operator>,
	words: HashMap<String, Vec<Operator>>,
	current_def_word: Option<String>,
}

impl VM{
	pub fn pop_data <'a> (&'a mut self) -> Operator {
		let data = self.data_stack.pop().expect("data stack is empty!");
		return data;
	}

	pub fn push_data <'a> (&'a mut self, op: Operator) {
		self.data_stack.push(op);
	}

	pub fn pop_op <'a> (&'a mut self) -> Operator {
		let op = self.op_stack.pop().expect("operand stack is empty!");
		return op;
	}

	pub fn push_op <'a> (&'a mut self, op: Operator) {
		match op {
			Operator::Define => {
				self.state = VMState::Definition;
				return;
			},
			Operator::EndDefine => {
				self.state = VMState::Execution;
				self.current_def_word = None;
				return;
			},
			_ => {}
		};
		let should_def = match self.state {
			VMState::Definition => true,
			VMState::Execution => false
		};
		if should_def {
			let current_def_word = self.push_def_word(op);
			self.current_def_word = Some(current_def_word);
		} else {
			self.op_stack.insert(0, op);
		}
	}

	fn push_def_word <'a> (&'a mut self, op: Operator) -> String{
		let wrapped = self.current_def_word.as_ref();
		let new_key = match wrapped {
			Some(w) => {
				self.words.get_mut(w).expect("error").insert(0, op);
				w.clone()
			},
			None => {
				let key = match op {
					Operator::Other(key) => key,
					_ => panic!("key is broken")
				};
				self.words.insert(key.clone(), Vec::new());
				key
			}
		};

		return new_key;
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
			Operator::Value(i) => {
				self.push_data(Operator::Value(i));
			},
			Operator::Print => self.print(),
			Operator::Other(s) => self.custom(s),
			x => panic!("cannot eval: {}", x.stringify())
		}
	}

	fn custom <'a> (&'a mut self, word: String) {
		let words = self.words_from_word(word);
		for op in words{
			self.op_stack.push(op.clone());
		}
	}

	fn words_from_word <'a> (&'a mut self, word: String) -> Vec<Operator>{
		let mut ret : Vec<Operator> = Vec::new();
		let ops = self.words.get(&word).expect("word not defined");
		for some_op in ops{
			ret.push(some_op.clone());
		}
		return ret;
	}

	fn pop_two_ints <'a> (&'a mut self) -> (i32, i32){
		let a = self.pop_one_int();
		let b = self.pop_one_int();
		(a, b)
	}

	fn pop_one_int <'a> (&'a mut self) -> i32 {
		let a = self.pop_data();
		match a {
			Operator::Value(j) => j,
			_ => panic!("can only add ints"),
		}
	}

	// Math Functions
	pub fn add <'a> (&'a mut self) {
		let (x, y) = self.pop_two_ints();
		self.push_data(Operator::Value(x + y));
	}

	pub fn subtract <'a> (&'a mut self) {
		let (x, y) = self.pop_two_ints();
		self.push_data(Operator::Value(x - y));
	}

	pub fn multiply <'a> (&'a mut self) {
		let (x, y) = self.pop_two_ints();
		self.push_data(Operator::Value(x * y));
	}

	pub fn divide <'a> (&'a mut self) {
		let (x, y) = self.pop_two_ints();
		self.push_data(Operator::Value(y / x));
	}

	pub fn modulus <'a> (&'a mut self) {
		let (x, y) = self.pop_two_ints();
		self.push_data(Operator::Value(x % y));
	}

	pub fn and <'a> (&'a mut self) {
		let (x, y) = self.pop_two_ints();
		self.push_data(Operator::Value(x & y));
	}

	pub fn or <'a> (&'a mut self) {
		let (x, y) = self.pop_two_ints();
		self.push_data(Operator::Value(x | y));
	}

	pub fn xor <'a> (&'a mut self) {
		let (x, y) = self.pop_two_ints();
		self.push_data(Operator::Value(x ^ y));
	}

	pub fn lshift <'a> (&'a mut self) {
		let (x, y) = self.pop_two_ints();
		self.push_data(Operator::Value(x << y));
	}

	pub fn rshift <'a> (&'a mut self) {
		let (x, y) = self.pop_two_ints();
		self.push_data(Operator::Value(x >> y));
	}

	pub fn abs <'a> (&'a mut self) {
		let x = self.pop_one_int();
		self.push_data(Operator::Value(if x < 0 {-x} else {x}));
	}

	pub fn max <'a> (&'a mut self) {
		let (x, y) = self.pop_two_ints();
		self.push_data(Operator::Value(if y > x {y} else {x}));
	}

	pub fn min <'a> (&'a mut self) {
		let (x, y) = self.pop_two_ints();
		self.push_data(Operator::Value(if y < x {y} else {x}));
	}

	pub fn invert <'a> (&'a mut self) {
		let x = self.pop_one_int();
		self.push_data(Operator::Value(!x));
	}

	pub fn negate <'a> (&'a mut self) {
		let x = self.pop_one_int();
		self.push_data(Operator::Value(-x));
	}

	//Stack Functions
	pub fn dup <'a> (&'a mut self) {
		let x = self.pop_data();
		self.push_data(x.clone());
		self.push_data(x.clone());
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
	EndDefine,

	//values and custom calls
	Value(i32),
	Other(String),
}

impl Operator {
	fn stringify(self) -> String {
		match self{
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
			Operator::EndDefine => format!(";"),
			Operator::Other(x) => format!("other {}", x),
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
		";" => Operator::EndDefine,

		//value or custom operator
		x => {
			let y: Option<i32> = FromStr::from_str(x).ok();
			match y{
				Some(i) => Operator::Value(i),
				None => Operator::Other(format!("{}", x)),
			}
		},
	}
}

fn main() {
	let x = io::stdin();
	let mut vm = VM {state: VMState::Execution,
					 op_stack: vec![],
					 data_stack: vec![],
					 words: HashMap::new(),
				     current_def_word: None};

	loop{
		let mut line: String = "".to_string();
		let _ = x.read_line(&mut line);

		if line.trim() == "" {break;}

		for o in line.split(" "){
			let b = parse(o.trim());
			vm.push_op(b)
		}

		while vm.op_stack.len() > 0 {
			vm.eval();
		}
	}
}
