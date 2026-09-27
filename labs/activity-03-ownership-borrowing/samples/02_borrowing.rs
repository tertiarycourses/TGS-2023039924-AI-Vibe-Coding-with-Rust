// Sample: shared borrows read, a mutable borrow changes one element.
fn total(stock: &[u32]) -> u32 {
    stock.iter().sum()
}

fn restock(level: &mut u32, amount: u32) {
    *level += amount;
}

fn main() {
    let mut stock = vec![5, 0, 7];
    println!("total before = {}", total(&stock)); // shared borrow
    restock(&mut stock[1], 10); // exclusive borrow of one element
    println!("stock after  = {stock:?}");
    println!("total after  = {}", total(&stock));
}
