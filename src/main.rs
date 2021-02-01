#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

use miniz_oxide::inflate::decompress_to_vec;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[no_mangle]
pub unsafe extern "C" fn main() -> i32 {
    libc::printf(
        decompress_to_vec(include_bytes!("ayaya.utf.ans.gz"))
            .unwrap()
            .as_ptr() as *const _,
    )
}

#[panic_handler]
unsafe fn panic(_: &core::panic::PanicInfo) -> ! {
    libc::exit(1);
}

#[alloc_error_handler]
unsafe fn panic_alloc(_: core::alloc::Layout) -> ! {
    libc::exit(2);
}
