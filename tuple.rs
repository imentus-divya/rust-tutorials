// A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
//  Tuples have a fixed length: once declared, they cannot grow or shrink in size.
fn main()
{
    let tup:(i32,char,&str)=(123,'D',"Divya");
    println!("first element - {}",tup.0);
    println!("second element - {}",tup.1);

    let(a,b,c)=tup;
    println!("a element - {a}");
    println!("b element - {b}");
    println!("c element - {c}");





}
