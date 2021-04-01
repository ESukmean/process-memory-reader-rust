use winapi::ctypes::*;
use winapi::shared::minwindef::*;
use winapi::um::winnt::HANDLE;
pub struct Reader {
    pid: DWORD,
    process_handle: HANDLE,
    module_handle: Option<winapi::um::winnt::HANDLE>,
    module_info: winapi::um::tlhelp32::MODULEENTRY32,
}

impl Reader {
    pub fn new(pid: DWORD) -> Self {
        let mut result = Self {
            pid: pid,
            process_handle: std::ptr::null_mut(),
            module_handle: None,
            module_info: winapi::um::tlhelp32::MODULEENTRY32::default(),
        };
        result.module_info.dwSize =
            std::mem::size_of::<winapi::um::tlhelp32::MODULEENTRY32>() as u32;

        return result;
    }

    pub fn open(&mut self) -> std::io::Result<()> {
        let process_handle = unsafe {
            winapi::um::processthreadsapi::OpenProcess(
                winapi::um::winnt::PROCESS_VM_READ,
                winapi::shared::minwindef::FALSE,
                self.pid,
            )
        };

        if process_handle.is_null() {
            return Err(std::io::Error::last_os_error());
        }

        self.process_handle = process_handle;
        return Ok(());
    }
    pub fn read(mut self) -> std::io::Result<Vec<u8>> {
        let success = unsafe {
            let initialized = self.module_handle.is_some();
            if initialized == false {
                self.module_handle
                    .replace(winapi::um::tlhelp32::CreateToolhelp32Snapshot(8, self.pid));
				println!("module {:?}", self.module_handle);
            }

            match initialized {
                true => winapi::um::tlhelp32::Module32First(
                    *self.module_handle.as_mut().unwrap(),
                    &mut self.module_info,
                ),
                false => winapi::um::tlhelp32::Module32Next(
                    *self.module_handle.as_mut().unwrap(),
                    &mut self.module_info,
                ),
            }
        };

        if success == 0 {
            return Err(std::io::Error::last_os_error());
        }

		println!("base {:?}", self.module_info.modBaseAddr as usize);
		println!("info {:?}", self.module_info.modBaseSize as usize);
		
        let mut buf = vec![0; self.module_info.modBaseSize as usize];
        let mut read = 0;
        unsafe {
            winapi::um::memoryapi::ReadProcessMemory(
                self.process_handle,
                self.module_info.modBaseAddr as LPCVOID,
                buf.as_mut_ptr() as LPVOID,
                buf.len(),
                &mut read,
            );
        }

        println!("{:?}", buf);
        return Ok(buf);
    }
}
