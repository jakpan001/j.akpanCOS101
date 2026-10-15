fn main() {
    // Create two vectors
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let x = vec![5, 6, 7, 8, 9, 10, 11];

    // Use a for loop to add the elements of the vectors pairwise.
    // x has 7 elements, so 0..6 keeps us safely inside both vectors.
    for index in 0..6 {
        let sum = v[index] + x[index];
        println!("{:?}", sum);
    }
}