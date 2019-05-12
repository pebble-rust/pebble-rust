use core::alloc::{GlobalAlloc, Layout};

pub struct Allocator;

extern {
    pub fn malloc(size: usize) -> *mut u8;
    pub fn calloc(count: usize, size: usize) -> *mut u8;
    pub fn realloc(ptr: *mut u8, size: usize) -> *mut u8;
    pub fn free(ptr: *mut u8);

    pub fn memcmp(ptr1: *const u8, ptr2: *const u8, num_bytes: usize) -> i32;
    pub fn memcpy(dest: *mut u8, src: *const u8, num_bytes: usize) -> *mut u8;
    pub fn memmove(dest: *mut u8, src: *const u8, num_bytes: usize) -> *mut u8;
    pub fn memset(dest: *mut u8, assign: i32, num_bytes: usize) -> *mut u8;
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