mod linked_list;
use linked_list::linked_list::LinkedList;

fn main() {
    println!("Hello, world!");

    let mut list = LinkedList::<i32>::new();

    list.push_front(4);
    list.push_front(5);

    println!("First value: {}", list.get(0).unwrap());

    list.remove(0);
    println!("First value: {}", list.get(0).unwrap());
}
