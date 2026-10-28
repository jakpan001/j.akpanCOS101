fn main(){

    // a list of nos
    let v = vec![15, 25, 35, 45, 55];
   let v2 = print_vector(v);
    println!("Inside Main [{}]",v2[0]);    // this line works now b

}
fn print_vector(x:Vec<i32>)->Vec<i32>{
    println!("Inside print_vector function {:?}",x);
    return x
}