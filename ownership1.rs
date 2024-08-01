// Each value in rust has a variable thats its owner

// here we are passing reference of a variable rather than sending variable
// function still has access to the data with reference 

fn main()
{
    let a=String::from("RUST");
    let len=cal_len(&a);
    println!("{len}")
}
fn cal_len(a:&String)->usize
{
    println!("string is {a}");
    a.len()
}