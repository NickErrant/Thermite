use std::io::stdin;
use std::convert::AsRef;

fn main() {
	
	let mut x = stdin();

	loop{
		let y: &mut String = &mut "".to_string();
		let _ = x.read_line(y);

	    println!("Hello, world! {}", match y.as_ref(){
	    	"+" => "addition!",
	    	"-" => "subtraction!",
	    	_ => "other!",
	    });
	}
}
