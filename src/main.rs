fn main() {
    let mut a=10;
    a=50;
    println!("Hello, world!");
    is_another();
    let x=-2;
    let y=x-1;
    let bla="bla bla bla";
    println!("the bla is {}",bla);
    println!("the value of y is {} and x is {}",y,x);
    println!("the value of a after getting mutated {}",a);
    if y>x{
        println!("yup the value of y is more");
    }
    else{
        println!("LUL")
    }
    let n=1;
    for n in 0..10{
        let mut z=n+1;
        println!("{z}");
    }
}
fn is_another() {
    let a=true;
    println!("{} is ",a);
}
//time to move to module imports and DSA