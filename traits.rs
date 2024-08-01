struct Pen
{
color:String,
price:u32,

}
struct Chocolate
{
name:String,
price:u32,
}

// implementation of trait
impl General for Chocolate{

fn display_info(&self)-> String
{
    format!("The name of Chocolate is {} and its price is {}",self.name,self.price)
}

}

impl General for Pen{

    fn display_info(&self)-> String
    {
        format!("The color of pen is {} and its price is {}",self.color,self.price)
    }
    
    }

trait General
{
    fn display_info(&self)-> String;
}
fn main()
{
 let choc_obj=Chocolate{
    name:String::from("Cadbury"),
    price:100
 };

 let pen_obj=Pen{
    color:String::from("red"),
    price:10
 };


 
 println!("display chocolate Info --> {}",choc_obj.display_info());
 println!("display pen Info --> {}",pen_obj.display_info());

}