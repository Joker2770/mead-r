use clap::{App, Arg};
use morse;
fn main() {
    let matches = App::new("mead-r")
        .version("0.1.0")
        .author("Jintao Yang <yjt950840@outlook.com>")
        .about("An application of morse-encode-and-decode rewrite in Rust programming language.")
        .arg(Arg::with_name("encode string")
                .short('e')
                .long("encode")
                .takes_value(true)
                .help("String to encode."))
        .arg(Arg::with_name("decode string")
                .short('d')
                .long("decode")
                .takes_value(true)
                .help("String to decode."))
        .get_matches();
    let enc_str = matches.value_of("encode string");
    if let Some(s) = enc_str.clone()
    {
        use_encode(s);
    }
    let dec_str = matches.value_of("decode string");
    if let Some(s) = dec_str.clone()
    {
        use_decode(s);
    }
}

fn use_encode(str: &str){
    match morse::encode::encode(str) {
        Ok(x) => println!("{}", x),
        Err(e) => {
            println!("The following chars were unsupported {:?}",
                     e.unsupported_characters)
        }
    }
    x
}

fn use_decode(str: &str) {
    match morse::decode::decode(str) {
        Ok(x) => println!("{}", x),
        Err(e) => {
            println!("The following chars were unsupported {:?}",
                     e.unsupported_characters)
        }
    }
}
