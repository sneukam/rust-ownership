fn main2() {
    println!("ownership!");
    let mut s: String = String::from("hello");
    s.push_str(", world!");  // push_str() appends a string literal to a String found in the heap
    println!("{}", s);

    let sx: String = String::from("hello2");
    let mut sj = sx;
    // println!("error: sx = {sx}");   // error here because sj now has ownership, and sx is invalid.
    // sj now owns the variable, and it alone will free the memory ('drop' in Rust) when it goes out of scope. There will be no double free of memory.
    // Rust never creates deep copies of the data by default. sj points to the same locaiton in memory as sx.

    // however, we can manually clone (deep copy).
    // we cannot clone sx because it is no longer valid, its value is now owned by sj.
    let sn: String = sj.clone();
    println!("sn has been cloned (deep copied). sn = {sn}");
    sj.push_str(" -> see can still use sj after deep copied.");
    println!("and because of that we are still allowed to use sj, b/c not taken ownership like sx was:\n\t{}", sj);
}

fn main3() {
    // working on more ownership examples
    let s: String = String::from("hello");

    take_ownership(s);
    println!("s is passed into the take ownership function, and it is a String created on the heap.");
    println!(" the value of s is 'moved' into the function, so any uses of s after the function thus attempt to 'borrow' the value when ownership has not been passed back to them.");
    println!("to avoid this, we can use some sort of annotation to tell the function it is borrowing the value, not owning it.");
    println!("thus, after the take_ownership() function returns, some_string (s) goes out of scope, and it is dropped by the function.");
    // println!("so using the value here will throw an error in the compiler: {}", s);

    let x: i32 = 5;
    make_copy(x);
    println!("since x is a scalar of known size, it is put on the stack in the new function call and implicitly copied. Scalar objects in rust already implement the copy trait: {}", x);

}

fn take_ownership(some_string: String) {
    println!("some_string() -> {}", some_string);
}

fn make_copy(some_int: i32) {
    println!("make_copy() -> {}", some_int);
}

fn main4() {

    // a String is returned and given to s1
    let s1: String = give_ownership();

    // s2 is allocated and given the hellow value
    let s2: String = String::from("hellow");

    // takes_and_gives_back() is given ownership of s2, and then it gives it back to s3. s2 is thus moved into s3 and can no longer be used.
    let s3: String = takes_and_gives_back(s2);

}

fn give_ownership() -> String {
    let some_string: String = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn main() {
    let s: String = String::from("a string!");
    let (s2, len) = calc_length(s);

    println!("Length of {} is {}", s2, len);

    println!("but the point being, it's a lot of work to have to return the value s that was moved to calc_length() in order to use it again.");
}

fn calc_length(some_string: String) -> (String, usize) {
    let len: usize = some_string.len();
    (some_string, len)
}
