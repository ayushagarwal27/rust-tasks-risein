fn main() {
    let string1 = "Hello".to_string();
    let string2 = " World!".to_string();
    let concatenated_string = concatenate_strings(&string1, &string2);
    println!("{concatenated_string}")
}

fn concatenate_strings(str1: &String, str2: &String) -> String {
    let mut res: String = String::from("");
    res.push_str(str1);
    res.push_str(str2);
    res
}