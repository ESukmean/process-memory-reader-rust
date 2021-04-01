mod read;
use read::*;

fn main() {
	let mut r: Reader = Reader::new(21472);
	r.open();
	println!("result {:?}", r.read());
}
