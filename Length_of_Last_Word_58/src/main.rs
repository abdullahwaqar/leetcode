use std::str::FromStr;

fn length_of_last_word(s: String) -> i32 {
    let buffer = s
        .split(" ")
        .map(|elem| elem.trim())
        .filter(|&elem| !elem.is_empty())
        .collect::<Vec<_>>();

    println!("{:?}", buffer);

    return buffer.last().unwrap().clone().len() as i32;
}

fn main() {
    let s = String::from_str("luffy is still joyboy    ").unwrap();

    let len = length_of_last_word(s);

    println!("{:?}", len);
    println!("yeah");
}
