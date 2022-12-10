use std::io::Write;
use support::parse_year_day;

fn download_input(year: u16, day: u16) -> String {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let client = reqwest::blocking::Client::builder().build().unwrap();
    let session_cookie = std::env::var("AOC_SESSION").expect("Environment variable not set, set AOC_SESSION to your cookie");

    let response = client
        .get(url)
        .header("Cookie", format!("session={}", session_cookie))
        .send()
        .unwrap();

    if response.status().is_success() {
        response.text().expect("no text")
    } else {
        panic!("{}", response.status());
    }
}

fn main() {
    let (year, day) = parse_year_day();
    let input = download_input(year, day);

    let mut out_path = std::env::current_dir().expect("couldn't get current dir");
    out_path.push("input.txt");
    let mut file = std::fs::File::options()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&out_path)
        .expect(&format!(
            "file {} couldn't be created",
            out_path.as_path().to_str().expect("can't stringify path")
        ));
    file.write_all(input.as_bytes()).expect("file didn't write");
}
