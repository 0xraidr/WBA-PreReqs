
// reference pointers - point to a resoruce in memory

pub fn run() {
    // primitive array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    // non primitive
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("Values: {:?}", (&vec1, vec2));

}