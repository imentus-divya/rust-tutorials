// Another way to have a collection of multiple values is with an array
// every element of an array must have the same type
fn main()
{
    let a=[1,2,3,4,5];
    println!("{}",a[0]);

    // write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array
    let a:[i32;4]=[10,2,3,4];
    println!("{}",a[0]);

    // an array to contain the same value for each element
    //  the initial value, followed by a semicolon, and then the length of the array in square brackets
    let a : [&str; 2]=["apple","banana"];
    println!("Fruit array 1st element: {}",a[0]);
    println!("Fruit array 2nd  element: {}",a[1]);

    let array=[50;5];
    println!("Elements of array : {:?}",array);



}