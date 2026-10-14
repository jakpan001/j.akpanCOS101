fn main() {
    // mutable array
    let mut colors = ["red", "green", "yellow", "white"];

    println!("\nOriginal array = {:?}", colors);

    // mutable slice: elements at index 1 and 2
    let sliced_colors = &mut colors[1..3];

    println!("First slice = {:?}", sliced_colors);

    // change the value at index 1 of the slice
    // (index 1 of the slice is index 2 of the original: "yellow")
    sliced_colors[1] = "purple";

    println!("Changed slice = {:?}", sliced_colors);

    // the slice borrowed colors mutably; its last use is above,
    // so we can print the original and see it changed too
    println!("Original array now = {:?}", colors);
}