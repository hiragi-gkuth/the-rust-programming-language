pub fn use_of_string() {
    let s = "Hello World".to_string();
    let utfs = "🧠̤¾".to_string();
    
    println!("{}", s);
    
    for s in utfs.bytes() {
        println!("{}", s);
    }
    for s in utfs.chars() {
        println!("{}", s);
    }
}