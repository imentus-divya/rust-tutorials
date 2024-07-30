fn main()
{
    let num:i32=7;
    check_condition(num);
    check_nested_conditions(num);
    let result= check_inline_condition(num);
    println!("result of inline checking is {}", result);

}
fn check_condition(num:i32)
{
    if num > 1
    {
        println!("{} is greater than  1",num);
    }
    else 
    {
        println!("{} is lesser than  1",num);
    }
}
fn check_nested_conditions(num:i32)
{

    if num%2==0
    {
        println!("{} is divisible by 2",num)
    }
    else if num%3==0
    {
        println!("{} is divisible by 3",num)

    }
    else if num%5 ==0
    {
        println!("{} is divisible by 5",num)

    }
    else
    {
        println!("{} is not divisible by 2/3/5",num)

    }
}
fn check_inline_condition(num:i32)-> i32
{
    let decision=true;
    // Remember that blocks of code evaluate to the last expression in them, and numbers by themselves are also expressions
    // the results of both the if  and the else  must be same. If the types are mismatched, we’ll get an error;
    if decision { num*num} else {num-1}
}