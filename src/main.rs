mod btree;
mod list;
use btree::BTree;
use list::List;

fn main() {
    let mut btree = BTree::<usize>::new();

    for i in 1..5 {
        btree.insert(i);
    }

    print!("Searching for 9:");
    println!(" {:?}", btree.search(9));

    print!("Searching for 10:");
    println!(" {:?}", btree.search(10));

    let mut list = List::<u8>::new();

    for _ in 1..100 {
        list.push_back(1);
    }

    for _ in 1..10 {
        list.pop_front();
    }
}
