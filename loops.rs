// Rust has three kinds of loops: loop, while, and for. Let’s try each one.

// The loop keyword -  execute a block of code over and over again forever or until you explicitly tell it to stop.
fn main()
{
    let mut i:i32=1;
    loop
    { 
        if i>5
        {
            println!("Loop stopped!");
            break;
        }
        println!("Thats an infinite loop ! --------counter is {} ",i);
        i+=1;

    }
}