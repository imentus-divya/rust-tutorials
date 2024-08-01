// There can only be one owner at a time 
fn main()
{
    let s1=String::from("StringValue");
    let s2=s1;
    // println!("{s1}") --->this will throw error as "StringValue" has now assigned to s2 and cant be used in s1
    println!("{s2}");

}