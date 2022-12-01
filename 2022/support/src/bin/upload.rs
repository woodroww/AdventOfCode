use support::parse_year_day;

fn post_answer(year: u16, day: u16, part: u16, answer: &str) -> String {

    let url = format!("https://adventofcode.com/{year}/day/{day}/answer");
    let client = reqwest::blocking::Client::builder().build().unwrap();
    let session_cookie = std::env::var("AOC_SESSION").unwrap();
    let body = serde_urlencoded::to_string(&serde_json::json!({
        "level": part,
        "answer": answer
    }))
    .unwrap();

    let response = client
        .post(url)
        .header("Cookie", format!("session={}", session_cookie))
        .body(body)
        .send()
        .unwrap();

    if response.status().is_success() {
        response.text().expect("no text")
    } else {
        panic!("{}", response.status());
    }
}


fn main() {

    let mut args = std::env::args();

    if args.len() != 3 {
        println!("usage: upload [part 1 or 2] answer");
        std::process::exit(0);
    }

    let part = args.nth(1).unwrap().parse::<u16>().unwrap();
    let answer = args.nth(0).unwrap();

    let (year, day) = parse_year_day();
    println!("uploading year:{} day:{} part:{} answer:{}", year, day, part, &answer);
    //let response = post_answer(year, day, part, &answer);

    //println!("{}", response);
}
