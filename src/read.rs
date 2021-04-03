use winapi::ctypes::*;
use winapi::shared::minwindef::*;
use winapi::um::winnt::HANDLE;

const INVAILD_HANDLE: i32 = -1;

pub struct ModuleReader {
	handle: winapi::um::winnt::HANDLE,
	is_first: bool,
}
impl ModuleReader {
	pub fn new(pid: DWORD) -> std::io::Result<Self> {
		let handle = unsafe { winapi::um::tlhelp32::CreateToolhelp32Snapshot(8, pid) };
		if handle as i32 == INVAILD_HANDLE {
			return Err(std::io::Error::last_os_error());
		}

		Ok(Self {
			handle,
			is_first: true,
		})
	}
	pub fn read(&mut self) -> std::io::Result<winapi::um::tlhelp32::MODULEENTRY32> {
		let mut result = winapi::um::tlhelp32::MODULEENTRY32::default();
		result.dwSize = std::mem::size_of::<winapi::um::tlhelp32::MODULEENTRY32>() as u32;

		let success = match self.is_first {
			true => {
				self.is_first = false;

				unsafe { winapi::um::tlhelp32::Module32First(self.handle, &mut result) }
			}
			false => unsafe { winapi::um::tlhelp32::Module32Next(self.handle, &mut result) },
		};
		// println!(
		// 	"name: {:?} / exe: {:?} / glb: {} / proc: {} / size: {} ",
		// 	String::from_utf8_lossy(
		// 		&(result.szModule
		// 			.to_vec()
		// 			.iter()
		// 			.map(|v| *v as u8)
		// 			.collect::<Vec<u8>>())
		// 	),
		// 	String::from_utf8_lossy(
		// 		&(result.szExePath
		// 			.to_vec()
		// 			.iter()
		// 			.map(|v| *v as u8)
		// 			.collect::<Vec<u8>>())
		// 	),
		// 	result.GlblcntUsage,
		// 	result.ProccntUsage,
		// 	result.modBaseSize
		// );

		if success as i32 == 0 {
			return Err(std::io::Error::last_os_error());
		}

		return Ok(result);
	}
}
impl Drop for ModuleReader {
	fn drop(&mut self) {
		unsafe {
			winapi::um::handleapi::CloseHandle(self.handle);
		}
	}
}
impl Iterator for ModuleReader {
	type Item = winapi::um::tlhelp32::MODULEENTRY32;

	fn next(&mut self) -> Option<Self::Item> {
		self.read().ok()
	}
}

pub struct MemoryReader {
	process_handle: HANDLE,
}

impl MemoryReader {
	pub fn new(pid: DWORD) -> std::io::Result<Self> {
		let process_handle = unsafe {
			winapi::um::processthreadsapi::OpenProcess(
				winapi::um::winnt::PROCESS_VM_READ,
				winapi::shared::minwindef::FALSE,
				pid,
			)
		};

		if process_handle as i32 == INVAILD_HANDLE {
			return Err(std::io::Error::last_os_error());
		}

		return Ok(Self { process_handle });
	}

	pub fn read(&mut self, addr: *mut u8, size: u32) -> std::io::Result<Vec<u8>> {
		let mut read_length = 0;
		let mut buf = vec![0u8; size as usize];

		let success = unsafe {
			winapi::um::memoryapi::ReadProcessMemory(
				self.process_handle,
				addr as *const c_void,
				buf.as_mut_ptr() as LPVOID,
				buf.len(),
				&mut read_length,
			)
		};

		// 벡터의 길이를 실제로 읽은 데이터 크기로 설정
		buf.truncate(read_length);

		match success == 0 {
			true => return Err(std::io::Error::last_os_error()),
			false => return Ok(buf),
		}
	}
}

impl Drop for MemoryReader {
	fn drop(&mut self) {
		unsafe {
			winapi::um::handleapi::CloseHandle(self.process_handle);
		}
	}
}

pub struct ModuleMemory {
	pub module: winapi::um::tlhelp32::MODULEENTRY32,
	pub data: Vec<u8>,
}

pub struct ProcessMemoryReader {
	module: ModuleReader,
	process: MemoryReader,
}
impl ProcessMemoryReader {
	pub fn new(pid: DWORD) -> std::io::Result<Self> {
		let process = MemoryReader::new(pid)?;
		let module = ModuleReader::new(pid)?;

		return Ok(Self { process, module });
	}
}
impl Iterator for ProcessMemoryReader {
	type Item = ModuleMemory;

	fn next(&mut self) -> Option<ModuleMemory> {
		match self.module.next() {
			None => {
				println!("none");
				return None;
			}
			Some(v) => {
				return self
					.process
					.read(v.modBaseAddr, v.modBaseSize)
					.map(|data| Self::Item { module: v, data })
					.ok();
			}
		}
	}
}
