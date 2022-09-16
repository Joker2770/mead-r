/*
MIT License

Copyright (c) 2022 Joker2770

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

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
