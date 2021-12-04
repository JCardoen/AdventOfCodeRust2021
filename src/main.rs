#[path = "day_one/one.rs"] mod day_one;
#[path = "day_two/two.rs"] mod day_two;
#[path = "day_three/three.rs"] mod day_three;
#[path = "day_four/four.rs"] mod day_four;

fn main() {
    /* DAY ONE
    let counted = day_one::star_one();
    let counted_two = day_one::start_two();

    println!("{}", counted);
    println!("{}", counted_two);


    // DAY 2
    //let position_one = day_two::star_one();
    //println!("{}", position_one);

    let position_two = day_two::star_two();

    println!("{}", position_two);*/

    // DAY THREE
    //let power_consumption = day_three::star_one();
    //println!("{}", power_consumption);

    //let power_consumption = day_three::star_two();
    //println!("{}", power_consumption);

    // DAY FOUR
    //et score_of_winning_board = day_four::star_one();
    //println!("{}", score_of_winning_board);

    let score_of_last_winning_board = day_four::star_two();
    println!("{}", score_of_last_winning_board);
}


