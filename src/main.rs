use std::ops::Add;

#[derive(Clone, Copy, Debug)]
struct Fruit {
    apples : i32,
    bananas : i32,
}

struct Vegetable<T> {
    onions : T,
    potatoes: T
}

#[derive(Clone, Copy, Debug)]
struct Kmh {
    value : u32,
}

#[derive(Clone, Copy, Debug)]
struct Km {
    value : u32, 
}

#[derive(Clone, Copy, Debug)]
struct Mph {
    value : u32,
}

#[derive(Clone, Copy, Debug)]
struct Miles {
    value : u32,
}

trait DistanceCalculator {
    type Distance;

    fn in_n_hours(&self, n : u32) -> Self::Distance;

}

impl DistanceCalculator for Kmh {
    
    type Distance = Km;
    
    fn in_n_hours(&self, n : u32) -> Km {
        Km {
            value : self.value * n,
        }
    }
}

impl DistanceCalculator for Mph {

    type Distance = Miles;

    fn in_n_hours(&self, n : u32) -> Miles {
        Miles {
            value : self.value * n 
        }
    }
}

trait Double {

    fn double(&self) -> Self;

}

impl Double for i32 {
    fn double(&self) -> Self {
        self * 2
    }
}

impl Double for String {

    fn double(&self) -> Self {
        format!("{} {}", self, self)
    }
}

fn print_vegetable<T: std::fmt::Display>(veg: &Vegetable<T>) {
    println!("potatoes: {}, bananas: {}", veg.potatoes, veg.onions);
}

impl<T: std::fmt::Display> Vegetable<T> {

    fn print(&self) {
        println!("I have {} potatoes and {} onions", self.potatoes, self.onions);
    }

    fn new(a : T, b : T) -> Self {
        Self {
            onions: a, 
            potatoes: b, 
        }
    }
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

fn quadruple<T : Double>(x : T) -> T {
    x.double().double()
}

impl<T: Double> Double for Vegetable<T> {
    fn double(&self) -> Self {
        Vegetable {
            onions: self.onions.double(),
            potatoes: self.potatoes.double(),
        }
    }
} 

fn print_fruit_1(fruit : Fruit) {
    println!("printing fruit once");
    fruit.print_fruit();
}

fn print_fruit_2(fruit : Fruit) {
    println!("printing fruit again");
    fruit.print_fruit();
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

    println!("Slices demo 1 {:?}", &numbers[1..5]);

    println!("Slices demo 2 {:?}", &numbers[1..]);

    println!("Slices demo 3 {:?}", &numbers[..3]);

    println!("Slices demo 4 {:?}", &numbers[..]);

    let veg : Vegetable<i32> = Vegetable {
        potatoes: 10, 
        onions: 10,
    };
    //println!("{} {}", veg.potatoes, veg.onions);
    print_vegetable(&veg);
    veg.print();

    let veg1 : Vegetable<String> = Vegetable::<String>::new("100".to_owned(), "1000".to_owned());
    veg1.print();

    let size = std::mem::size_of::<Vegetable<String>>();
    println!("size is {}", size);

    println!("double of 20 is {}", 20_i32.double());
    println!("{}", "hello".to_string().double());

    println!("quadruple of 100 is {} quadruple of hello is {}", quadruple(100), quadruple("Hello".to_owned()));

    veg.double().print();

    println!("Cloning fruit");
    fruit.clone().print_fruit();
    print_fruit_1(fruit);
    print_fruit_2(fruit);
    println!("{:?}", fruit);

    println!("using std ops {}", 2_i32.add(5));

    let kmh = Kmh {
        value : 100,
    };

    let mph = Mph {
        value : 20
    };

    let km = kmh.in_n_hours(5);

    let mp = mph.in_n_hours(8);

    println!("at speed {:?}  we can go for {:?}", kmh, km);
    println!("at speed {:?}  we can go for {:?}", mph, mp);
}
