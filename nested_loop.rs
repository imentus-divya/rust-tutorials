fn main()
{
    'outerLoop:loop
    {
        println!("entered outer loop");
        'innerLoop:loop
        {
            println!("entered inner loop");
            break 'outerLoop
        }
    }
}