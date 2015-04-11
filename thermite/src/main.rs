use std::io::stdin;

fn main() {
	
	let mut x = stdin();

	loop{
		let mut y: String = "".to_string();
		let _ = x.read_line(&mut y);

		let z = y.chars().next();
	    println!("Hello, world! {}", match z{
	    	Some(b) => match b{
	    		'+' => "addition!",
	    		_ => "other!",
	    	},
	    	None => "none!",
	    });
	}
}
