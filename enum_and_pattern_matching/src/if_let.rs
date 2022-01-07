fn use_of_if_let() {
    let config_max = Some(3u8);
    if let Some(x) = config_max {
        println!("PThe maximum is configured to be {}", x);
    }
}

fn main() {
    use_of_if_let();
}