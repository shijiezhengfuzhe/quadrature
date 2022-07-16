use std::ops::Mul;
struct Rect<T,U>
{
    height:T,
    weight:U,  
}
impl<T,U> Rect<T,U>{
    fn area(&self) -> T
    where T:Mul<U,Output = T> + Copy,
          U:Mul<T,Output = T> +Copy
    {
        self.height* self.weight
    }
}
fn main(){
    let  rect=Rect{height:100,weight:56};
    println!("{}",rect.area());
}