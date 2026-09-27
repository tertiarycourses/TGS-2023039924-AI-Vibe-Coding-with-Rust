// Sample: moving ownership, cloning, and Copy types.
fn consume(items: Vec<String>) -> usize {
    items.len() // `items` is dropped at the end of this function
}

fn main() {
    let items = vec![String::from("cable"), String::from("charger")];
    let copy = items.clone(); // explicit deep copy
    let count = consume(items); // ownership moves into `consume`
                                // println!("{:?}", items); // error[E0382]: borrow of moved value
    println!("consumed {count} items; the clone still has {copy:?}");
    let n: u32 = 5;
    let m = n; // integers are Copy: both stay usable
    println!("n = {n}, m = {m}");
}
