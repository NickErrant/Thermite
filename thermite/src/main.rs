use std::io;

enum Operator {
	Add,
	Subtract,
	Push,
	Pop,
	None,
}

fn parse(s: &str) -> Operator {
	match s {
		"+" => Operator::Add,
		"-" => Operator::Subtract,
		"pop" => Operator::Pop,
		_ => Operator::None,
	}	
}

fn process(s: String) {
	for o in s.split(" "){
		let b = parse(o.trim());
		println!("{}",match b {
			Operator::Add => "addition!",
			Operator::Subtract => "subtraction!",
			Operator::Pop => "popped!",
			_ => "other!",
		});
	}
}

fn main() {
	
	let mut x = io::stdin();

	loop{
		let mut y: String = "".to_string();
		let _ = x.read_line(&mut y);

	    process(y);
	}
}
