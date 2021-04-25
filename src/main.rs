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

#[panic_handler]
unsafe fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[alloc_error_handler]
unsafe fn panic_alloc(_: Layout) -> ! {
    loop {}
}
