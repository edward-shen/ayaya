#![no_std]
#![no_main]
#![feature(alloc_error_handler, naked_functions)]

use core::alloc::{GlobalAlloc, Layout};
use core::arch::asm;
use core::panic::PanicInfo;
use core::ptr::null_mut;
use miniz_oxide::inflate::core::{decompress, inflate_flags, DecompressorOxide};

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
    loop {}
}

#[alloc_error_handler]
unsafe fn panic_alloc(_: Layout) -> ! {
    loop {}
}

// Linux Library Globals #######################################################

/// We need to define our own _start function because the C runtime inserts a
/// lot of stuff that isn't needed.
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
#[cfg(all(target_os = "windows"))]
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

const BUFFER_SIZE: usize = 68865;

#[no_mangle]
fn m() {
    let mut decompressor = DecompressorOxide::new();
    decompressor.init();
    let mut out = [0u8; BUFFER_SIZE];
    decompress(
        &mut decompressor,
        include_bytes!("ayaya.utf.ans.gz"),
        &mut out,
        0,
        inflate_flags::TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF,
    );
    w(&out);
}

#[no_mangle]
#[inline(always)]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn w(out: &[u8; BUFFER_SIZE]) {
    unsafe {
        asm!(
            "syscall",
            in("rax") 1, // SYS_write
            in("rdi") 1, // stdout
            in("rsi") out.as_ptr(),
            in("rdx") out.len(),
        );
    }
}

#[no_mangle]
#[inline(always)]
#[cfg(target_os = "windows")]
fn w(out: &[u8; BUFFER_SIZE]) {
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
