// Structs are similar to tuples, discussed in “The Tuple Type” section, in that both hold multiple related values.

// To define a struct, we enter the keyword struct and name the entire struct. A struct’s name should describe the significance of the pieces of data being grouped together.

// To use a struct after we’ve defined it, we create an instance of that struct by specifying concrete values for each of the fields

// key: value pairs, where the keys are the names of the fields and the values are the data we want to store in those fields. 

// The struct definition is like a general template for the type, and instances fill in that template with particular data to create values of the type.
struct Student
{
 first_name:String,
 last_name:String,
 age:u32,
 roll_no:u32,

}
fn update_student_details(first_name:String,last_name:String)->Student
{
//  function that returns a Student instance 
    Student{
        first_name:String::from(first_name),
        last_name:String::from(last_name),
        age:90,
        roll_no:123,
    }


}
fn main()
{
    let mut st1=Student
    {
        first_name:String::from("Divya"),
        last_name:String::from("Soni"),
        age:22,
        roll_no:591,
    };
    println!("student first name --> {}",st1.first_name);
    println!("student last name --> {}",st1.last_name);
    println!("student age --> {}",st1.age);
    println!("student roll number --> {}",st1.roll_no);

    // If we want to  change any field's value, then we need to make our object mutable 'mut'
    // If the instance is mutable, we can change a value by using the dot notation and assigning into a particular field

    st1.first_name=String::from("Divyaaaa");
    println!("student's updated first name --> {}",st1.first_name);
    
    update_student_details("first_name".to_string(),"last_name".to_string());
    println!("student's modified first name --> {}",st1.first_name);



}