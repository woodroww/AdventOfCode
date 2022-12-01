use std::fs;

fn main() {

    let contents = fs::read_to_string("input.txt")
        .expect("Couldn't read file");
    let mut num_strings: Vec<&str> = contents
        .split('\n').collect();
    num_strings.pop();

    //println!("num_strings {:?}", num_strings);

    let previous_count = 25; // 5 in test 25 in real data
    let mut nums = Vec::with_capacity(num_strings.len());
    for num in num_strings {
        if num.len() != 0 {
            nums.push(num.parse::<i64>().unwrap());
        }
    }
    //println!("nums {:?}", nums);

    let mut not_found = 0;
    for location in previous_count..nums.len() {
        let mut possible = Vec::with_capacity(previous_count * 2);
        for i in location - previous_count..location {
            for j in i..location  {
                if i != j {
                    //println!("indicies {} {}, {} + {} = {}", i, j, nums[i], nums[j], nums[i] + nums[j]);
                    possible.push(nums[i] + nums[j]);
                }
            }
        }
        //println!("possible {:?}\n", possible);

        if !possible.contains(&nums[location]) {
            not_found = nums[location];
            break;
        }
    }

    println!("not_found {:?}", not_found);
}
























