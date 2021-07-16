#[no_mangle]
pub fn sum(addr: i32, size: i32) -> i64 {
    let v_size = 1000000000;
    let mut v = vec![0u8];
    for _ in 0..v_size {
        v.push(0);
    }

    let mut u = vec![0u8];
    for _ in 0..v_size {
        u.push(0);
    }

    let mut result = 0i64;
    let addr = addr as *mut u8;
    let input = unsafe { std::slice::from_raw_parts(addr, size as _) };
    for i in 0..(size as usize) {
        result += (input[i]) as i64;
    }

    // test mem allocation on stack

    result
}

#[no_mangle]
pub fn add(adder1: i32, adder2: i32) -> i32 {
    adder1 + adder2
}
