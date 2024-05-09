pub fn in_range(lower_bound: i32, higher_bound: i32) -> impl Fn(&i32) -> Option<String> {
    move |e| {
        if *e < lower_bound || *e > higher_bound {
            Some(format!(
                "Value is not in range {}..{}",
                lower_bound, higher_bound
            ))
        } else {
            None
        }
    }
}
