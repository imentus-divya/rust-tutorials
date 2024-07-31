// The while keyword can be used to run a loop while a condition is true.
fn main()
{
    let mut i:i32=10;
    let ten:i32=10;

    while i>0
    {
        let product =ten*i;
        println!(" {} * {} = {} ",ten,i,product);
        i-=1;
    }
}