use std::mem;

fn allocate_with_libc() {
    unsafe {
        let my_num: *mut i32 = libc::malloc(mem::size_of::<i32>() as libc::size_t) as *mut i32;
        if my_num.is_null() {
            panic!("allocate_with_libc failed.");
        }

        *my_num = 42;
        assert_eq!(*my_num, 42);

        // free memory - it is not automatic
        libc::free(my_num as *mut libc::c_void);
    }
}

fn allocate_memory_with_rust() {
    use std::alloc::{alloc, dealloc, Layout};

    unsafe {
        let layout = Layout::new::<u16>();
        let ptr = alloc(layout);

        *ptr = 42;
        assert_eq!(*ptr, 42);

        // free memory - it is not automatic
        dealloc(ptr, layout);
    }
}
fn main() {
    allocate_with_libc();

    allocate_memory_with_rust();
}
