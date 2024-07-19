mod strings;
mod maps;
mod problems;

fn main() {
    let list = [1,4,3,7,9,5,3,6,5,2,4,3,4,6,7,8,1,6,5];
    let (mean, median, mode) = problems::mean_median_mode(&list[..]);

    println!("mean: {}, median: {}, mode: {}", mean, median, mode);
    
    let a = "apple";
    let f = "first";
    
    
    println!("{}", problems::pig_latin(a));
    println!("{}", problems::pig_latin(f));
}