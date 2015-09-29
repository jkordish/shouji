extern crate curl;
extern crate serde_json;

use self::curl::http;
use std::str::from_utf8;
use std::io::prelude::*;
use std::fs::File;
use mods::*;

pub fn export(server: &str, port: &str, key: &str, file: &str, verbose: bool) {

    let mut file = File::create(file).unwrap();

    // build url from input values
    // format the url depending on if a key is set
    let url: String =
        if key == "" {
            format!("http://{}:{}/v1/kv/?recurse", server, port)
        } else {
            format!("http://{}:{}/v1/kv/{}/?recurse", server, port, key)
        };

    // verbose: print out the connection url string
    if verbose {
        println!("Attempting connection to {}", &url);
    }

    // make connection
    let resp = http::handle()
        .get(url)
        .exec()
        .unwrap();

    // expect a 200 code or error with return code
    if resp.get_code() != 200 {
        println!("Unable to handle HTTP response code {}", resp.get_code())
    }

    // verbose: print out the response code, headers, and body
    if verbose {
        println!("code={}; headers={:?}; body={}",
            resp.get_code(), resp.get_headers(), from_utf8(resp.get_body()).unwrap());
    }

    // make body from the response body from the server
    let body = from_utf8(resp.get_body()).unwrap();

    // map json body to our backend Struct
    let json: Vec<ValueData> = serde_json::from_str(&body[..]).unwrap();

    let output = decode_json(&json).unwrap();

    file.write_all(output.as_bytes()).unwrap()
}