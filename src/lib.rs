use std::any::Any;

#[no_mangle]
pub extern fn Java_org_tests_Test_hello(env: *const Any, jclass: *const Any) {
    println!("hello from rust");
}

#[no_mangle]
pub extern fn Java_org_tests_Test_sum(env: *const Any, jclass: *const Any, a: i32, b: i32) -> i32 {
    return a + b;
}