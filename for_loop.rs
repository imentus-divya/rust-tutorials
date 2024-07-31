// range notation a..b. This yields values from a (inclusive) to b
fn main()
{
    for i in 1..5
    {
        println!(" element in range {i}")
    }
     
     println!("\n");

    let ar=[13,4,78,90];
    for i in ar.iter()
    {
        println!("Array element - {i}")
    }

    println!("\n");

    let ar1=[100;4];
    for i in ar1.into_iter()
    {
        println!("Array element - {i}")
        
    }



}