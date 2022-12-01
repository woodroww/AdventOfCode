pub fn parse_year_day() -> (u16, u16) {
    let dir = std::env::current_dir().expect("couldn't get current dir");
    let file = dir.file_name().unwrap().to_str().unwrap();

    println!(
        "you are in {}",
        dir.as_path().to_str().expect("can't string")
    );
    if !file.starts_with("day") {
        panic!("you are not in a day folder");
    }

    let day = file
        .strip_prefix("day")
        .expect("error removing day from string")
        .parse::<u16>()
        .expect("error parsing u8 from day string");

    println!("you are on day {}", day);

    let year = dir
        .parent()
        .expect("file has no parent")
        .file_name()
        .expect("couldn't get file_name")
        .to_str()
        .expect("couldn't convert osstring to string")
        .parse::<u16>()
        .expect("couldn't parse string into a year");

    println!("you are in year {}", year);
    (year, day)
}
