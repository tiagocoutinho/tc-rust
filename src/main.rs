
use clap::{App, Arg};

fn main() {
    let app = App::new("grrs")
        .version("1.0")
        .about("A mini grep in rust")
        .author("Tiago Coutinho")
        .arg(Arg::with_name("pattern")
             .short("p")
             .long("pattern")
             .takes_value(true)
             .required(true)
             .help("pattern to match"))
        .arg(Arg::with_name("path")
             .required(true)
             .long("path")
             .takes_value(true)
             .help("file to inspect"));
    let matches = app.get_matches();

    let pattern = matches.value_of("pattern").unwrap();
    let path = matches.value_of("path").unwrap();

    println!("pattern=\"{:}\" path=\"{:}\"", pattern, path);
}
