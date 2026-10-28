fn main(){
    let v = vec![101, 250, 330, 400];
    // vector v owns the object in heap

    let v2 = v;
    //here two variables holds the heap value

    println!("{:?}",v2);
}