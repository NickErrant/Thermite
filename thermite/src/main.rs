use std::io::stdin;

enum Operator {
	Add,
	Subtract,
	Push,
	Pop,
	None,
}

fn parse(s: String) -> Operator {
	let c = s.chars().next();
	match c {
		Some(i) => match i {
		'+' => Operator::Add,
		'-' => Operator::Subtract,
		_ => Operator::Push,
		},
		None => Operator::None,
	}
	
}

fn main() {
	
	let mut x = stdin();

	loop{
		let mut y: String = "".to_string();
		let _ = x.read_line(&mut y);

		let z = parse(y);
	    println!("{}", match z{
	    	Operator::Add => "Addition!",
	    	Operator::Subtract => "Subtraction!",
	    	_ => "other!",
	    });
	}
}
