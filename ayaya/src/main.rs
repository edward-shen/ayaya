#![no_std]
#![no_main]
#![feature(alloc_error_handler, naked_functions, slice_as_chunks)]

use core::alloc::{GlobalAlloc, Layout};
use core::arch::asm;
use core::panic::PanicInfo;
use core::ptr::null_mut;

// Rust globals ################################################################

#[global_allocator]
static ALLOC: DummyAllocator = DummyAllocator;
struct DummyAllocator;
unsafe impl GlobalAlloc for DummyAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[panic_handler]
unsafe fn panic(_: &PanicInfo) -> ! {
    asm!("hlt", options(nomem, nostack, noreturn));
}

#[alloc_error_handler]
unsafe fn panic_alloc(_: Layout) -> ! {
    asm!("hlt", options(nomem, nostack, noreturn));
}

// Linux Library Globals #######################################################

// Keeping this function name a single letter saves us bytes with no_mangle
#[naked]
#[no_mangle]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub unsafe extern "C" fn a() -> ! {
    asm!(
        "call m",
        "mov rax, 60", // SYS_exit
        "mov rdi, 0",  // exit code
        "syscall",
        options(noreturn)
    )
}

// Window Library Globals ######################################################

#[no_mangle]
#[cfg(target_os = "windows")]
pub unsafe extern "C" fn mainCRTStartup() -> u32 {
    m();
    return 0;
}

#[link(name = "Kernel32")]
#[cfg(target_os = "windows")]
extern "stdcall" {
    fn SetConsoleOutputCP(codepage: u32) -> i32;
    fn GetStdHandle(nStdHandle: i32) -> *const i8;
    fn WriteFile(
        hFile: *const i8,
        lpBuffer: *const u8,
        nNumberOfBytesToWrite: u32,
        lpNumberOfBytesWritten: usize,
        lpOverlapped: usize,
    ) -> i32;
}

#[no_mangle]
#[cfg(target_os = "windows")]
pub static _fltused: i32 = 0;

#[link(name = "libcmt")]
#[cfg(target_os = "windows")]
extern "C" {}

// Program start ###############################################################

#[cfg(not(feature = "smaller"))]
mod mapping;

#[cfg(not(feature = "smaller"))]
const CHAR_BUF_SIZE: usize = 3;

/// Stolen from u8::to_string, except without allocating
// Keeping this function name a single letter saves us bytes with no_mangle
#[cfg(not(feature = "smaller"))]
#[no_mangle]
fn b(mut n: u8, buf: &mut [u8; CHAR_BUF_SIZE]) -> &[u8] {
    let mut index = 0;
    if n >= 10 {
        if n >= 100 {
            let mut_ref = unsafe { buf.get_unchecked_mut(index) };
            *mut_ref = b'0' + n / 100;
            index += 1;
            n %= 100;
        }
        let mut_ref = unsafe { buf.get_unchecked_mut(index) };
        *mut_ref = b'0' + n / 10;
        index += 1;
        n %= 10;
    }
    let mut_ref = unsafe { buf.get_unchecked_mut(index) };
    *mut_ref = b'0' + n;
    index += 1;
    &buf[..index]
}

// Keeping this function name a single letter saves us bytes with no_mangle
#[no_mangle]
fn m() {
    #[cfg(not(feature = "smaller"))]
    {
        let chunks = unsafe { include_bytes!("data").as_chunks_unchecked::<2>() };
        let mut buf = [0u8; CHAR_BUF_SIZE];

        for [code, character] in chunks {
            match code {
                // Use previous color (don't write new color)
                254 => {
                    let real_char =
                        unsafe { mapping::CHAR_MAPPING.get_unchecked(*character as usize) };
                    w(real_char.encode_utf8(&mut buf).as_bytes());
                }
                // Write newline
                255 => {
                    w(&[0x1b]);
                    w(b"[0m\n");
                }
                // Get color from mapping
                other => {
                    let (fg, bg) = unsafe { mapping::COLOR_MAPPING.get_unchecked(*other as usize) };
                    w(&[0x1b]);
                    w(b"[38;5;");
                    w(b(*fg, &mut buf));
                    w(b";48;5;");
                    w(b(*bg, &mut buf));
                    w(b"m");
                    let real_char =
                        unsafe { mapping::CHAR_MAPPING.get_unchecked(*character as usize) };
                    w(real_char.encode_utf8(&mut buf).as_bytes());
                }
            }
        }
    }

    #[cfg(feature = "smaller")]
    {
        w(include_bytes!("ayaya.utf.ans"));
    }
}

#[no_mangle]
#[inline(always)]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn w(out: &[u8]) {
    unsafe {
        asm!(
            "syscall",
            in("rax") 1, // SYS_write
            in("rdi") 1, // stdout
            in("rsi") out.as_ptr(),
            in("rdx") out.len(),
            out("rcx") _,
            out("r11") _,
            lateout("rax") _,
            options(nostack),
        );
    }
}

#[no_mangle]
#[inline(always)]
#[cfg(target_os = "windows")]
fn w(out: &[u8]) {
    unsafe {
        SetConsoleOutputCP(65001);
        WriteFile(
            GetStdHandle(-11 /* STD_OUTPUT_HANDLE */),
            out.as_ptr(),
            out.len() as u32,
            0,
            0,
        );
    }
}
