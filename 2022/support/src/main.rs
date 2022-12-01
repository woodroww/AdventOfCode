use dotenv;

fn get_input(year: u16, day: u16) -> String {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let client = reqwest::blocking::Client::builder().build().unwrap();
    let session_cookie = dotenv::var("session").unwrap();

    let response = client
        .get(url)
        .header(
            "Cookie",
            format!("session={}", session_cookie),
        )
        .send()
        .unwrap();

    if response.status().is_success() {
        response.text().expect("no text")
    } else {
        panic!("{}", response.status());
    }
}

// /Users/matt/prog/adventOfCode/2022/support/target/debug/support

fn main() {
    dotenv::dotenv().ok();

    let dir = std::env::current_dir().expect("couldn't get current dir");
    let file = dir.file_name().unwrap().to_str().unwrap();
    
    println!("you are in {}", dir.as_path().to_str().expect("can't string"));
    if !file.starts_with("day") {
        panic!("you are not in a day folder");
    }

    let day = file.strip_prefix("day")
        .expect("error removing day from string")
        .parse::<u8>()
        .expect("error parsing u8 from day string");

    println!("you are on day {}", day);

    let year = dir.parent()
        .expect("file has no parent")
        .file_name()
        .expect("couldn't get file_name")
        .to_str()
        .expect("couldn't convert osstring to string")
        .parse::<u16>()
        .expect("couldn't parse string into a year");

    println!("you are in year {}", year);

}


