fn main() {
    // Vec<T>
    // heap-allocated
    
    let mut numbers: Vec<i32> = Vec::new();

    numbers.push(10);
    numbers.push(20);
    numbers.push(30);
    numbers.push(40);

    println!("numbers: {:?}", numbers);
    
   // creating a vector using the vector macro

   let numbers2 = vec![1,2,3,4,5];

    println!("numbers2: {:?}", numbers2);


    let mut fruits = vec!["apple", "banana", "orange"];
    println!("fruits: {:?}", fruits);

    //add an element
   fruits.push("grape");
    println!("fruits: {:?}", fruits);

    // remove an element
    let removed_fruit = fruits.pop();
    println!("{:?}, Removed: {:?}", fruits, removed_fruit);
}
