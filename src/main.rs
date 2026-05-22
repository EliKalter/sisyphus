fn main() {
    let mut stone_count = 1;

    loop {
        let oddity = stone_count % 2;
        if oddity == 0 {
            println!("The sun is setting");
        } else {
            println!("The sun is rising");
        }
        roll_stone(stone_count);
        stone_count += 1;
        if stone_count == 10 {
            break;
        }
    }
}

fn roll_stone(stone_number: i32) {
    println!("The stone count is {}", stone_number);
}
