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

    fn new(apples : i32, bananas : i32) -> Self {
        Self {
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

fn greet(text : String) {
    println!("{}", text)
}

fn print_sum(numbers : &[i32]) -> i32 {
    let mut i = 0;
    let n = numbers.len();
    let mut sum = 0;
    while i < n {
        sum += numbers[i];
        i += 1;
    }
    return sum
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

    let first_name : String = "hello".to_owned();
    let last_name : &str = " world";
    let full_name = first_name + last_name ;
    //println!("{}", full_name);
    greet(full_name.clone());
    greet(full_name);
    let message : String = format!("Hello my name is {} and I am {} years old", "Anwesh", 31);
    println!("{}", message);

    let numbers : [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    let mut num_index = 0;

    let numbers_length = numbers.len();

    while num_index < numbers_length {
        println!("number at {} is {}", num_index, numbers[num_index]);
        num_index += 1;
    }

    println!("sum of numbers is {}", print_sum(&numbers));

    let new_num : &[i32] = &[5, 2, 1, 8, 9];

    println!("sum of new_num {}", print_sum(new_num));

    let mut nums_vector : Vec<i32> = vec![1, 4, 2, 3, 11, 6];

    nums_vector.push(10);

    println!("{:?}", nums_vector);


}
