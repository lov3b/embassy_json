//                _                                _
//  ___ _ __ ___ | |__   __ _ ___ ___ _   _       (_)___  ___  _ __
// / _ \ '_ ` _ \| '_ \ / _` / __/ __| | | |      | / __|/ _ \| '_ \
//|  __/ | | | | | |_) | (_| \__ \__ \ |_| |      | \__ \ (_) | | | |
// \___|_| |_| |_|_.__/ \__,_|___/___/\__, |____ _/ |___/\___/|_| |_|
//                                    |___/_____|__/

use std::{fs, io::Read};
fn main() {
    println!("I am alive. Now go and fuck yourself");

    let countries = fs::read_to_string("countries.html")
        .expect("There is no countries.html file");
    for i in countries.lines(){
        println!("NEW LINE: {}",&i);
    }

}
