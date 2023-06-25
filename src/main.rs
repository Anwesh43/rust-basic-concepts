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

fn deref_demo() {
    let x : i32 = 6;
    let mut y : i32 = 7;
    let z : &mut i32 = &mut y;
    *z -= 1;
    assert_eq!(x, y);
    println!("success");
}

fn reference_demo() {
    let x : i32 = 100;
    println!("{} at {:p}", x, &x)
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
    reference_demo();
    deref_demo();
}
