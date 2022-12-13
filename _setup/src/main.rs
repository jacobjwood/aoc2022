//use reqwest;
use reqwest::blocking::Client;
use reqwest::header::{
    HeaderMap, HeaderValue, COOKIE,
};
use std::io::Write;

fn main() -> std::io::Result<()> {

    // Get command line arg
    let args : Vec<String> = std::env::args().collect();
    let day = args[1].to_owned();

    println!("Generating input for Day {}", day);

    // Read in session cookie and generate header
    let session_cookie = std::fs::read_to_string("./session_cookie.txt").expect("Could not read session_cookie.txt");
    let cookie_header = HeaderValue::from_str(&format!("session={}", session_cookie.trim())).unwrap();
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, cookie_header);

    // Build the client
    let client = Client::builder()
        .default_headers(headers)
        .build().unwrap();

    // Set the URL 
    let url = format!("https://adventofcode.com/2022/day/{}/input", day);

    // Get response from URL
    let resp = client
        .get(&url)
        .send()
        .and_then(|response| response.error_for_status())
        .and_then(|response| response.text())
        .unwrap();

    // Write contents of response to file
    let input_path = format!("day{}/input.txt", day);
    let mut file = std::fs::File::create(input_path)?;

    // Remove trailing newline character in request response
    let mut resp = resp.chars();

    match resp.to_owned().last().expect("Response text is empty.") {
        '\n' => {
            resp.next_back();
        },
        _ => (),
    }

    write!(file, "{}", resp.as_str())?;

    Ok(())
}
