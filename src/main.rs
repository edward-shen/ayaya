#![no_std]
#![no_main]
#![feature(alloc_error_handler, asm, naked_functions)]

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use miniz_oxide::inflate::core::{decompress, inflate_flags, DecompressorOxide};

#[global_allocator]
static ALLOC: DummyAllocator = DummyAllocator;
struct DummyAllocator;
unsafe impl GlobalAlloc for DummyAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[cfg(all(target_os = "linux", target_arch="x86_64"))]
#[no_mangle]
#[naked]
pub unsafe extern "C" fn _start() -> ! {
    asm!(
        "call main",
        "mov rax, 60", // SYS_exit
        "mov rdi, 0",  // exit code
        "syscall",
        options(noreturn)
    )
}

#[cfg(all(target_os = "windows"))]
#[no_mangle]
pub unsafe extern "C" fn mainCRTStartup() -> u32 {
    main();
    return 0;
}

#[no_mangle]
fn main() {
    let mut decompressor = DecompressorOxide::new();
    decompressor.init();
    let mut out = [0u8; 68865];
    decompress(
        &mut decompressor,
        include_bytes!("ayaya.utf.ans.gz"),
        &mut out,
        0,
        inflate_flags::TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF,
    );
    write(&out);
}

#[cfg(all(target_os = "linux", target_arch="x86_64"))]
#[no_mangle]
#[inline(always)]
fn write(out : &[u8; 68865])
{
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

#[cfg(target_os = "windows")]
#[no_mangle]
#[inline(always)]
fn write(out : &[u8; 68865])
{
    unsafe {
        SetConsoleOutputCP(65001);
        WriteFile(
            GetStdHandle(-11 /* STD_OUTPUT_HANDLE */),
            out.as_ptr(),
            out.len() as u32,
            0,
            0
        );
    }
}

#[panic_handler]
unsafe fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[alloc_error_handler]
unsafe fn panic_alloc(_: Layout) -> ! {
    loop {}
}

#[cfg(target_os = "windows")]
#[link(name = "Kernel32")]
extern "stdcall" {
    fn SetConsoleOutputCP(codepage : u32) -> i32;
    fn GetStdHandle(nStdHandle : i32) -> *const i8;
    fn WriteFile(hFile : *const i8, lpBuffer : *const u8, nNumberOfBytesToWrite : u32, lpNumberOfBytesWritten : usize, lpOverlapped : usize) -> i32;
}
#[cfg(target_os = "windows")]
#[no_mangle]
pub static _fltused: i32 = 0;

#[cfg(target_os = "windows")]
#[link(name = "libcmt")]
extern "C" {}