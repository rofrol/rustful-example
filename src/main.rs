#[macro_use]
extern crate rustful;

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate hyper;

use hyper::Url;
use hyper::client::Client;
use hyper::header::Connection;

use std::error::Error;

// You'll also need to bring Read into scope to have access to read_to_string
// http://stackoverflow.com/questions/29214963/unable-to-read-file-contents-to-string-result-does-not-implement-any-method-in
// use std::io::Read;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn main() {
    env_logger::init().unwrap();


    // println!("# Menu");
    // for node in document.find(Attr("id", "hmenus").descendant(Name("a"))) {
    //    println!("{} ({:?})", node.text(), node.attr("href").unwrap());
    // }
    // println!("");

    let client = Client::new();

    // error: cannot borrow immutable local variable `res` as mutable
    // https://github.com/hyperium/hyper/issues/333

    // let url = "http://thepiratebay.org/top/207";
    // let url = Url::parse("http://thepiratebay.org/top/207").unwrap();
    let url = match Url::parse("http://thepiratebay.org/top/207") {
        Ok(url) => url,
        Err(_) => panic!("Uh oh."),
    };
    let mut res = client.get(url)
        .header(Connection::close())
        .send()
        .unwrap();
    assert_eq!(res.status, hyper::Ok);
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let path = Path::new("thepiratebay--top--207.html");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    match file.write_all(body.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
