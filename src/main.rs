fn main() {
    let list_1 = vec!['a', 'k', 'r', 'm'];
    let list_2 = vec!['a', 'o', 'i', 'e', 'u'];
    println!("The Largest Character is : {}", largest_char(& list_1));
    println!("The Largest Character is : {}", largest_char(& list_2));

}
fn largest_char(data: &[char]) -> char {
    let mut largest = data[0];
    for &check in data.iter() {
        if check > largest {
            largest = check;
        }
    }
    largest 
}