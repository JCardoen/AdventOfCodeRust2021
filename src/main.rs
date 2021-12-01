#[path = "day_one/one.rs"] mod day_one;


fn main() {
    let counted = day_one::star_one();
    let counted_two = day_one::start_two();

    println!("{}", counted);
    println!("{}", counted_two);

}


