extern crate sbp;

use sbp::messages::SBP;
use sbp::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("src/28-220741.sbp");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("Can't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut tow &[i64]
    let mut parser = sbp::parser::Parser::new();
    loop {
        match parser.parse(&mut file) {
            Ok(SBP::MsgPosLLHGnss(x)) => println!("{} {}", x.tow, x.h_accuracy),
            Ok(_) => (),

            Err(Error::NotEnoughData) => (),
            Err(Error::UnrecoverableFailure) => (),
            Err(Error::ParseError) => (),
            Err(Error::IoError(ref x)) if x.kind() == std::io::ErrorKind::TimedOut => (),

            Err(e) => {
                println!("{:?}", e);
                break;
            }
        }
    }
}
