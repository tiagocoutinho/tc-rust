fn t_str() {
    // primitive type: string slice (aka str)
    let str1 = "Hello, 🌏 as str1!"; // type: &str

    // pointer to string slice
    let str1_ptr = str1.as_ptr(); // type: *const u8

    // doing &str1[1..10] would result in a panic attack since index 10 would not be
    // at a char boundary
    let str1_slice = &str1[1..11]; // type: &str

    let bytes1 = str1_slice.as_bytes(); // type: &u[8]

    println!("str1:");
    println!("  display={}", str1);
    println!("  debug={:?}", str1);
    println!("  ptr={:?}", str1_ptr);
    println!("  [2..11]={:?}", str1_slice);
    println!("  bytes={:?}", bytes1);
}

fn t_bytes() {
    let b1 = b"Hello, World!"; // type: &[u8]
    println!("bytes1:");
    println!("  debug={:?}", b1);
    let mut b2: [u8; 13] = *b"Hello, world!"; // type: [u8]
    b2[7] = b'W';
    assert_eq!(b1, &b2);
    println!("bytes2:");
    print!("  debug=");
    for c in &b2 {
        print!("{}, ", c);
    }
}

fn main() {
    t_str();
    println!();
    t_bytes();
}
