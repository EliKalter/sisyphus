use rand::Rng;

struct Stone {
    number: i32,
    weight: u32,
    name: Option<String>,
}

impl Stone {
    fn is_heavy(&self) -> bool {
        self.weight >= 1000
    }

    fn roll_stone(&self) {
        if self.is_heavy() {
            println!("Wow this stone is so so heavy!");
            println!("Carrying stone of weight {}", self.weight);
        }
        match &self.name {
            None => println!("This is a nameless stone!"),
            Some(n) => println!("The stone name is {}", n),
        }
        match self.number {
            1 => println!("Starting the day!"),
            5 => println!("Halfway there!"),
            9 => println!("Almose done for now"),
            _ => {
                let oddity = self.number % 2;
                if oddity == 0 {
                    println!("The sun is setting");
                } else {
                    println!("The sun is rising");
                }
            }
        }
        println!("The stone count is {}", self.number);
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let mut stones = Vec::new();

    for ind in 1..4 {
        let rand_weight: u32 = rng.gen_range(0..5000);
        let rand_bool: bool = rng.gen_bool(0.5);
        let new_stone: Stone = Stone {
            number: ind,
            weight: rand_weight,
            name: if rand_bool {
                Some(format!("Stone number {}", ind))
            } else {
                None
            },
        };
        stones.push(new_stone);
    }

    stones
        .iter()
        .filter(|s| s.is_heavy())
        .for_each(|s| s.roll_stone());
}
