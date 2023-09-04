fn main() {
    println!("ownership!");
    let mut s: String = String::from("hello");
    s.push_str(", world!");  // push_str() appends a string literal to a String
    println!("{}", s);

    let sx: String = String::from("hello2");
    let sj = sx;
    // println!("error: sx = {sx}");   // error here because sj now has ownership, and sx is invalid.
    // sj now owns the variable, and it alone will free the memory ('drop' in Rust) when it goes out of scope. There will be no double free of memory.
    // Rust never creates deep copies of the data by default. sj points to the same locaiton in memory as sx.

    // however, we can manually clone (deep copy).
    // we cannot clone sx because it is no longer valid, its value is now owned by sj.
    let sn: String = sj.clone();
    println!("sn has been cloned (deep copied). sn = {sn}");
}
