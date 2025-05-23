fn main() {
    let name = String::from("Dibas K Borborah");
    let ans = first_word(&name);
    println!("{:?}", ans);
}

fn first_word(s: &String) -> &str {
    for (i, x) in s.chars().enumerate() {
        if x == ' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
