pub fn get_length(str_: &String) -> usize {
    println!("{} inside the function", str_);
    str_.len()
}

fn main() {
    let sample_string = String::from("Hello");
    let result = get_length(&sample_string);
    assert_eq!(result, 5, "Values doesn't match");
    println!("{} outside the function", sample_string);
}
