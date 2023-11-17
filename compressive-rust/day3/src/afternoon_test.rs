#[test]
fn unsafe_point() {
    let mut num = 5;
    let r1 = &mut num as *mut i32;
    let r2 = r1 as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        *r1 = 10;
        println!("r2 is: {}", *r2);
    }
}

#[allow(unused)]
static mut COUNTER: u32 = 0;

#[allow(unused)]
fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

#[test]
fn unsafe_change_const() {
    add_to_counter(42);
    unsafe {
        println!("COUNTER: {COUNTER}");
    }
}

#[repr(C)]
#[allow(unused)]
union MyUnion {
    i: u8,
    b: bool,
}

#[test]
fn unsafe_union() {
    let u = MyUnion { i: 42 };
    println!("int: {}", unsafe { u.i });
    println!("bool: {}", unsafe { u.b }); // Undefined behavior!
}

#[allow(unused)]
unsafe fn swap(a: *mut u8, b: *mut u8) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

#[test]
fn unsafe_extra_fn() {
    let mut a = 42;
    let mut b = 66;

    unsafe {
        swap(&mut a, &mut b);
    }

    println!("a = {}, b = {}", a, b);
}

extern "C" {
    #[allow(unused)]
    fn abs(input: i32) -> i32;
}

#[test]
fn unsafe_extern_fn() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}


