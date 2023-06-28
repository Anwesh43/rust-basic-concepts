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

impl Fruit {

    fn print_fruit(&self) {
        println!("{} bananas {} apples", self.bananas, self.apples);
    }

    fn increase_fruit(&mut self) {
        self.bananas += 5;
        self.apples += 10;
    }

    fn price(&self) -> i32 {
        self.bananas * 10 + self.apples * 20
    }

    fn new(apples : i32, bananas : i32) -> Fruit {
        Fruit {
            apples: apples, 
            bananas: bananas,
        }
    }
}

fn new_fruit_for_methods() -> Fruit {
    Fruit {
        apples: 100, 
        bananas: 50,
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
    reference_demo();
    deref_demo();
    let mut fruit : Fruit = new_fruit_for_methods();
    fruit.print_fruit();
    println!("price of fruit is {}", fruit.price());
    fruit.increase_fruit();
    fruit.print_fruit();
    println!("price of fruit is {}", fruit.price());

    let new_fruit : Fruit = Fruit::new(126, 213);
    new_fruit.print_fruit();

}
