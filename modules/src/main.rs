pub mod cooking
{
    pub mod making_tea{
        pub fn adding_tea(){
            println!(" adding_tea");
        }
        pub fn adding_water()
        {
            println!(" adding_water");
            
        }
    }
}

fn main() {

    // ABSOLUTE PATH
    crate::cooking::making_tea::adding_tea();

    // RELATIVE PATH
    cooking::making_tea::adding_water();

}
