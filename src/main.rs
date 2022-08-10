use tuple_sort::tuple_sort::tuple_sort;

fn main() {
    //The numbers to be ordered
    let mut numbers = vec![48,23,78,67,89,22,33,44];
    
    tuple_sort(&mut numbers);
    
    println!("Result: {:?}",numbers);
}