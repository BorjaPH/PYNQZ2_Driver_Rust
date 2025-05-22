use libc::{c_void, mmap, munmap, O_RDWR, PROT_READ, PROT_WRITE, MAP_SHARED};
use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;
use std::ptr;

pub const GPIO_BASE_ADDR: usize = 0x41250000;
pub const GPIO_MAP_SIZE: usize = 0x10000;

pub struct Gpio {
    ptr: *mut u32,
    map_size: usize,
}

impl Gpio {
    pub fn new() -> Result<Self, String> {
        let dev_mem = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/mem")
            .map_err(|e| format!("No se pudo abrir /dev/mem: {}", e))?;

        let fd = dev_mem.as_raw_fd();

        let gpio_ptr = unsafe {
            mmap(
                ptr::null_mut(),
                GPIO_MAP_SIZE,
                PROT_READ | PROT_WRITE,
                MAP_SHARED,
                fd,
                GPIO_BASE_ADDR as i32,
            )
        };

        if gpio_ptr == libc::MAP_FAILED {
            return Err("Fallo al mapear memoria".into());
        }

        Ok(Gpio {
            ptr: gpio_ptr as *mut u32,
            map_size: GPIO_MAP_SIZE,
        })
    }

    pub fn write(&self, value: u32) {
        unsafe {
            ptr::write_volatile(self.ptr, value);
        }
    }
}

impl Drop for Gpio {
    fn drop(&mut self) {
        unsafe {
            munmap(self.ptr as *mut c_void, self.map_size);
        }
    }
}
