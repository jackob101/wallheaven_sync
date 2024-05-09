pub fn bool_mapper(input: &str) -> Option<bool> {
    match input {
        "y" | "Y" => Some(true),
        "n" | "N" => Some(false),
        _ => None,
    }
}

pub fn bool_mapper_with_default(default: bool) -> impl Fn(&str) -> Option<bool> {
    move |input| match input {
        "y" | "Y" => Some(true),
        "n" | "N" => Some(false),
        "" => Some(default),
        _ => None,
    }
}

pub fn i32_mapper(input: &str) -> Option<i32> {
    input.parse::<i32>().ok()
}
