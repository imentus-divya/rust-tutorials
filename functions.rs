// Expression - Anything that returns a value / no semicolon
// Statements - Anything that does not returns a value / has semi colon
// hoisting - can call function placed anywhere in your code
fn main()
{
    let a:i32=5;
    another_fn(a);
        let x={
            let price:i32=100;
            let qty:i32=2;
            price*qty
        };
        println!("Result of price * qty is : {} ",x);


        println!("Add function written inline -- {}.",add(10,5));

}
 fn another_fn(num:i32)
 {
    let result:i32=num*num;
    println!("{result}");
 }
 
//  Functions can return values to the code that calls them. We don’t name return values, but we must declare their type after an arrow (->). 
 fn add (a:i32,b:i32)-> i32
 {
    a+b
 }