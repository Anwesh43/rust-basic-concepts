struct Fruit {
    apples : i32,
    bananas : i32, 
}

fn print_fruit(fruit : &Fruit) {
    println!("number of apples {}, number of bananas {}", fruit.apples, fruit.bananas)
}

fn increase_fruit(fruit : &mut Fruit) -> Fruit {
    fruit.apples += 10;
    fruit.bananas += 10;
    Fruit {
        apples: fruit.apples, 
        bananas: fruit.bananas
    }   
}

fn main() {
    println!("Hello, world!");
    let mut fruit : Fruit = Fruit {
        apples : 10, 
        bananas : 15,
    };
    print_fruit(&fruit);
    let fruit : Fruit = increase_fruit(&mut fruit);
    print_fruit(&fruit);
}
