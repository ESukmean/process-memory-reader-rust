mod read;
use read::*;

fn main() {
	let mut r: Reader = match Reader::new(21472) {
        Err(e) => panic!("프로세스 오픈에 실패했습니다. {:?}", e),
        Ok(r) => r
    };

	println!("result {:?}", r.read());
}
