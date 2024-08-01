fn main()
{
    let owner:u32=5;

    // mutable reference
    let reference=&owner;
    println!("reference--> {reference}");
    println!("owner-->  {owner}");
    
    println!("\n--------------MUTABLE REFERENCE---------------\n");
    // to create mutable reference, where reference can modify the actual data of owner we need to make 'owner' mutable and reference 'mutable' as well
    let mut owner2=100;
    let reference2=&mut owner2;
    println!("owner2 before --> {owner2}");
    // println!("reference2 before --> {reference2}");

    // derefencing is done using '*'
    *reference2+=100;
    println!("owner2 after --> {owner2}");
    // println!("reference2 after--> {reference2}");


}