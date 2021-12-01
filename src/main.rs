//                _                                _
//  ___ _ __ ___ | |__   __ _ ___ ___ _   _       (_)___  ___  _ __
// / _ \ '_ ` _ \| '_ \ / _` / __/ __| | | |      | / __|/ _ \| '_ \
//|  __/ | | | | | |_) | (_| \__ \__ \ |_| |      | \__ \ (_) | | | |
// \___|_| |_| |_|_.__/ \__,_|___/___/\__, |____ _/ |___/\___/|_| |_|
//                                    |___/_____|__/

use std::{fs, io::Read};
fn main() {
    println!("I am alive. Now go and fuck yourself");

    let countries = fs::read_to_string("countries.html").expect("There is no countries.html file");
    for i in countries.lines() {
 
        // If we aren't on a line that contains desired link then skip
        if !i.contains("href=") {
            continue;
        }

        let i = i.trim().replace("<a class=\"entry-title\" href=\"", "");
        let mut firstquote = i.find("\"").unwrap();
        let (link, _) = i.split_at(firstquote);


    }
}

struct Embassy<'a> {
    country: &'a str,
    link: &'a str,
    mail: Option<&'a str>,
}
