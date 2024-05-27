
fn main() {
    for i in 1..10 {
        for j in 1..i+1{
            print!("{} x {} = {:2}   ",j,i,i*j);
        }
        println!();
    }
}