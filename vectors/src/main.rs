fn main() {
    // Vec<T>
    // heap-allocated
    
    // let mut numbers: Vec<i32> = Vec::new();

    // numbers.push(10);
    // numbers.push(20);
    // numbers.push(30);
    // numbers.push(40);

    // println!("numbers: {:?}", numbers);
    
   // creating a vector using the vector macro

   // let numbers2 = vec![1,2,3,4,5];

    // println!("numbers2: {:?}", numbers2);


    // let mut fruits = vec!["apple", "banana", "orange"];
    // println!("fruits: {:?}", fruits);

    //add an element
   // fruits.push("grape");
    // println!("fruits: {:?}", fruits);

    // remove an element
    // let removed_fruit = fruits.pop();
    //println!("{:?}, Removed: {:?}", fruits, removed_fruit);
    
   // accessing and modifying elements
   
   // let numbers = vec![100,200,300,400,500];

    //let second = numbers[1];
    //println!("The second element is: {:?}", second);
    
    // match numbers.get(5) {
    //     Some(value) => println!("The value at index 5 is: {}", value),
    //     None => println!("No value at index 5")
    // }
    
   let animals = vec!["dog", "cat", "rabbit"];

    for animal in &animals {
        println!("{animal}");
    }

    let mut numbers = vec![1,2,3,4,5];

    for number in &mut numbers {
        *number *= 2;
    }

    println!("{:?}", numbers);
}
