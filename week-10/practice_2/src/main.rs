fn main(){
    let v = vec![10,20,30];
    //vector v owns the object in heap
    let v2 = v;   // moves ownership to v2

    let v3 = display(v2);
    //v2 is being moved to display and v2 is invalidated

    println!("In main {:?}",v3);
    //This will print because v2 has been changed to display() which now contain the value

}
fn display(v: Vec<i32>) -> Vec<i32>{
    println!("inside display{:?}",v);
    return v
}