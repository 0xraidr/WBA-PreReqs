
use std::mem;
pub fn run(){
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

//re-assign value
numbers[2] = 20;

// add on to vector

numbers.push(5);
numbers.push(6);

// pop off last value

numbers.pop();

println!("{:?}", numbers);

// get single val

println!("single value: {}", numbers[0]);

//get vector length

println!("vector length: {}", numbers.len());

// vector are stack allocated
println!("vector occupies {} bytes", mem::size_of_val(&numbers));

// get slice

let slice: &[i32] = &numbers[0..2];
println!("Slice: {:?}", slice);

// loop through vector values
for i in numbers.iter(){
    println!("Number: {}", i);
}

// loop & mutate values
for x in numbers.iter_mut(){
    *x *= 2;
}
    println!("Numbers Vec: {:?}", numbers);
}