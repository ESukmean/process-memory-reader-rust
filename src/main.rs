mod read;
use read::*;

fn main() {
	let pid = 21232;

	let r: ProcessMemoryReader = match ProcessMemoryReader::new(pid) {
		Err(e) => panic!("프로세스 오픈에 실패했습니다. {:?}", e),
		Ok(r) => r,
	};

	let mut nth = 0;
	for data in r {
		println!(
			"name: {:?} / exe: {:?} / glb: {} / proc: {} / size: {}\ndata: {:?}",
			String::from_utf8_lossy(
				&(data
					.module
					.szModule
					.to_vec()
					.iter()
					.take_while(|x| **x != 0i8)
					.map(|v| *v as u8)
					.collect::<Vec<u8>>())
			),
			String::from_utf8_lossy(
				&(data
					.module
					.szExePath
					.to_vec()
					.iter()
					.take_while(|x| **x != 0i8)
					.map(|v| *v as u8)
					.collect::<Vec<u8>>())
			),
			data.module.GlblcntUsage,
			data.module.ProccntUsage,
			data.module.modBaseSize,
			data.data
		);
		// write_to_file(&format!("{}-{}", pid, nth), &buf);
		nth += 1;
		std::thread::sleep_ms(1000);
	}
}
fn write_to_file(name: &str, buf: &[u8]) -> std::io::Result<()> {
	use std::fs::File;
	use std::io::prelude::*;

	let mut file = File::create(name)?;
	file.write_all(buf)?;

	Ok(())
}
