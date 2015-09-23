extern crate curl;
extern crate rustc_serialize;

use self::curl::http;
use std::str::from_utf8;
use ::serde_json::Value;

pub fn list(server: &str, port: u16, key: &str, verbose: bool ) {

    // build url from input values
    let url = format!("http://{}:{}/v1/kv/{}/?recurse", server, port, key);

    // verbose: print out the connection url string
    if verbose {
        println!("Attempting connection to {}", url);
    }

    // make connection
    let resp = http::handle()
        .get(url)
        .exec()
        .unwrap();

    // expect a 200 code or error with return code
    if resp.get_code() != 200 {
        println!("Unable to handle HTTP response code {}", resp.get_code());
        return;
    }

    // verbose: print out the response code, headers, and body
    if verbose {
        println!("code={}; headers={:?}; body={}",
            resp.get_code(), resp.get_headers(), from_utf8(resp.get_body()).unwrap());
    }

    let body = from_utf8(resp.get_body()).unwrap();
    let mut json: Value = ::serde_json::from_str(body).unwrap();
    let json = ::serde_json::to_string_pretty(&mut json);

    println!("{}", json.unwrap());
}
