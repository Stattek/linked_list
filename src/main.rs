mod data_structure;
use data_structure::linked_list::LinkedList;

fn main() {
    let mut list = LinkedList::<i32>::new();

    list.push_front(4);
    list.push_front(5);

    println!("First value: {}", list.get(0).unwrap());

    list.remove(0).unwrap();
    println!("First value: {}", list.get(0).unwrap());
    list.remove(0).unwrap();
    // now we have an empty list at this point

    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_back(4);

    list.remove(1).unwrap();
    list.add_at(0, 0).unwrap();
    list.remove(0).unwrap();
}
