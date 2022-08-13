extern "C" {
    pub fn log_number(number: u32);
}

#[no_mangle]
pub fn alloc(len: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(len);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    return ptr;
}

#[no_mangle]
pub unsafe fn array_sum(ptr: *mut u8, len: usize) -> u8 {
    let vec = Vec::from_raw_parts(ptr, len, len);
    vec.iter().sum()
}

#[no_mangle]
pub unsafe fn to_upper(ptr: *mut u8, len: usize) -> *mut u8 {
    let vec = Vec::from_raw_parts(ptr, len, len);
    let mut string = String::from_utf8_unchecked(vec).to_uppercase();
    let ptr = string.as_mut_ptr();
    std::mem::forget(string);
    ptr
}

#[no_mangle]
pub unsafe fn dealloc(ptr: *mut u8, size: usize) {
    let data = Vec::from_raw_parts(ptr, size, size);
    std::mem::drop(data);
}

fn main() {
    // create a `Vec<u8>` as input
    let input = vec![1 as u8, 2, 3, 4, 5];
    // call the `alloc` function
    let ptr = alloc(input.len());
    let res: u8;
    unsafe {
        // copy the contents of `input`into the buffer
        // returned by `alloc`
        std::ptr::copy(input.as_ptr(), ptr, input.len());
        // call the `array_sum` function with the pointer
        // and the length of the array
        res = array_sum(ptr, input.len());
    }
    // print the result
    println!("Result: {:#?}", res);
}
