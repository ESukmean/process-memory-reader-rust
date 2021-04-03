# Process-Memory-Reader (Rust)
Windows에서 특정 프로세스(pid)의 메모리를 읽는 프로그램(사실상 라이브러리)입니다.

winapi 0.3.9에서 작동함을 확인하였습니다.


## 구조
크게 3가지 부분으로 나뉘어집니다.
 - Module Reader
 - Memory Reader
 - ProcessMemory Reader

### 모듈 리더 (read::ModuleReader)
프로세스의 메모리를 읽으려면 각 부분(모듈)의 base address를 구해야합니다. 모듈의 base address를 구하기 위해 필요한 부분입니다. 
```Rust
let moduleR = ModuleReader::new(pid);
for x in moduleR {
	/// 모듈 정보가 x에 담겨있음 
	/// https://docs.rs/winapi/0.3.9/winapi/um/tlhelp32/struct.MODULEENTRY32.html
}
```

### 프로세스 리더 (read::MemoryReader)
프로세스의 메모리를 실제로 읽는 부분입니다. 프로세스와 메모리 주소, 그리고 읽을 길이를 지정해 주면 됩니다.
```Rust
let processR = MemoryReader::new(pid)
processR.read(addr, size)
// MemoryReader.read(addr: *mut u8, size: u32) -> std::io::Result<Vec<u8>>
```

### 프로세스-모듈 리더 (read::ProcessMemoryReader)
위의 프로세스 리더와 모듈 리더의 결합체입니다. pid를 지정해 주면 모듈 정보를 읽고 각 부분을 메모리 리더가 읽습니다.
```Rust
let reader: ProcessMemoryReader = match ProcessMemoryReader::new(pid) {
	Err(e) => panic!("프로세스 오픈에 실패했습니다. {:?}", e),
	Ok(r) => r,
};
for data in reader {
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
}
```

WinAPI의 변경이 없으면 별도의 업데이트는 없을 예정입니다.
# License
Mit 라이센스를 따릅니다.