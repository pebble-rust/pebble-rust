use core::alloc::{GlobalAlloc, Layout};

pub struct Allocator;

extern {
    fn malloc(size: usize) -> *mut u8;
    fn realloc(ptr: *mut u8, size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        free(ptr);
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        realloc(ptr, new_size)
    }
}

#[no_mangle]
extern fn __rust_alloc(size: usize) -> *mut u8 {
    unsafe {malloc(size)}
}

#[no_mangle]
extern fn __rust_dealloc(ptr: *mut u8) {
    unsafe {free(ptr)}
}

#[no_mangle]
extern fn __rust_realloc(ptr: *mut u8, new_size: usize) -> *mut u8 {
    unsafe {realloc(ptr, new_size)}
}

#[no_mangle]
extern fn __rust_alloc_zeroed() -> *mut u8 {
    unsafe {malloc(0)}
}