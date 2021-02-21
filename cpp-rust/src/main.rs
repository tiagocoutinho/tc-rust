#[cxx::bridge(namespace = "rango")]
mod ffi {
    unsafe extern "C++" {
        include!("cpp-rust/include/rango.h");

        type Rango;

        fn new_rango(dev_name: &str) -> UniquePtr<Rango>;
        fn read_attribute(&self, attr_name: &str) -> f64;
    }
}

fn main() {
    let r = ffi::new_rango("a/b/c");
    println!("Hello, world {}!", r.read_attribute("hello"));
}
