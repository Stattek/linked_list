//! Defines a simple linked list. Used to learn how to utilize the
//! `unsafe` keyword.
use std::ptr::NonNull;

/// Linked List struct that can hold any type of value.
///
/// We essentially treat the first node of the LinkedList as the head.
/// It will never contain a value, it will just point to the rest of the list.
pub struct LinkedList<StoreType> {
    value: Option<Box<StoreType>>,
    // next is NonNull because we need raw pointers
    //to be able to navigate through the linked list mutably.
    next: Option<NonNull<LinkedList<StoreType>>>,
}

impl<StoreType> LinkedList<StoreType> {
    /// Create a new empty list.
    pub fn new() -> Self {
        Self {
            value: None,
            next: None,
        }
    }

    /// Remove the value at the specified index.
    ///
    /// # Params
    /// - `idx` - The index to remove the value at.
    ///
    /// # Returns
    /// - `Ok(())` if the value could be removed, `Err(())` otherwise.
    pub fn remove(&mut self, idx: usize) -> Result<(), ()> {
        let mut cur_node; // our current value

        if idx == 0 {
            // remove the head of the list
            if let Some(temp_val) = self.next {
                cur_node = temp_val.as_ptr();
            } else {
                return Err(()); // expected a value but got none
            }

            // SAFETY: cur_node is always Some value
            unsafe {
                // set to what the cur_node has as its next node.
                // Could be Some or None
                self.next = (*cur_node).next;

                // set the next node of the current node to None so we do not accidentally deallocate the rest of the list
                (*cur_node).next = None;

                // drop this node
                drop(Box::from_raw(cur_node));
            }
        } else {
            // remove some node in the middle/end of the list

            if let Some(temp_val) = self.next {
                cur_node = temp_val.as_ptr();
            } else {
                return Err(()); // fail, empty list
            }

            let mut cur_idx = 0;
            // keep going until we are at the value right before
            while cur_idx < idx - 1 {
                // SAFETY: cur_node is always Some value
                unsafe {
                    if let Some(temp_val) = (*cur_node).next {
                        cur_node = temp_val.as_ptr();
                    } else {
                        return Err(()); // fail, expected a value to be here and there wasn't
                    }
                }
                cur_idx += 1;
            }

            // now we have the node right before the value to remove
            let node_to_remove;

            // SAFETY: cur_node is always Some value
            unsafe {
                if let Some(temp_val) = (*cur_node).next {
                    node_to_remove = temp_val.as_ptr();
                } else {
                    return Err(()); // expected the next value to exist, but it doesn't
                }

                // we have a value to point to
                (*cur_node).next = (*node_to_remove).next;

                // so we do not accidentally deallocate the rest of the list
                (*node_to_remove).next = None;

                // drop the node to remove now
                drop(Box::from_raw(node_to_remove));
            }
        }

        Ok(())
    }

    /// Pushes a value at the beginning of the list.
    /// Sets this value as the new head.
    ///
    /// # Params
    /// - `value` - The value to push to the front of the list.
    pub fn push_front(&mut self, value: StoreType) {
        // allocate on the heap
        let new_node = Box::new(LinkedList {
            value: Some(Box::new(value)),
            next: None,
        });
        if self.next.is_none() {
            // this is the new head of the list
            self.next = Some(Box::leak(new_node).into());
        } else {
            // the list has something next, so we need to do some magic

            let mut new_node_ptr: NonNull<LinkedList<StoreType>> = Box::leak(new_node).into();
            // SAFETY: new_node_ptr is always valid
            unsafe {
                // new node should point to the current head
                new_node_ptr.as_mut().next = self.next;
            }
            // head is now the new pointer
            self.next = Some(new_node_ptr);
        }
    }

    /// Pushes a value at the end of the list.
    ///
    /// # Params
    /// - `value` - The value to push back.
    pub fn push_back(&mut self, value: StoreType) {
        if self.next.is_none() {
            // empty list, push to the front

            let new_node = Box::new(LinkedList {
                value: Some(Box::new(value)),
                next: None,
            });
            self.next = Some(Box::leak(new_node).into());
        } else {
            unsafe {
                // we already checked self.next to be some value
                let mut cur_node_ptr = self.next.unwrap_unchecked().as_ptr();

                // keep going until we are at the last node
                while (*cur_node_ptr).next.is_some() {
                    // we already checked that the next value is something
                    cur_node_ptr = (*cur_node_ptr).next.unwrap_unchecked().as_ptr();
                }

                // allocate on the heap
                let new_node = Box::new(LinkedList {
                    value: Some(Box::new(value)),
                    next: None,
                });
                // this is the new tail of the list
                (*cur_node_ptr).next = Some(Box::leak(new_node).into());
            }
        }
    }

    /// Gets the node at the index provided, or None if it couldn't be found.
    ///
    /// # Returns
    /// - Reference to `Some` value if it could be found, `None` otherwise.
    fn get_node_at(&self, idx: usize) -> &Option<NonNull<LinkedList<StoreType>>> {
        let mut cur_node = &self.next;
        let mut cur_idx = 0;

        // keep going until we have our value or we reach a none
        while cur_idx < idx && cur_node.is_some() {
            // SAFETY: cur_node is always Some value
            unsafe {
                cur_node = &cur_node.unwrap_unchecked().as_ref().next;
            }
            cur_idx += 1;
        }

        cur_node
    }

    /// Adds a value at the index provided. Places the new value before
    /// the existing value in the list.
    ///
    /// # Param
    /// - `value` - The value to add.
    /// - `idx` - The index in the list to add the value at.
    ///
    /// # Returns
    /// - `OK(())` if the value could be added, `Err(())` otherwise.
    pub fn add_at(&mut self, value: StoreType, idx: usize) -> Result<(), ()> {
        if idx == 0 {
            // push front
            self.push_front(value);
        } else {
            // get the node before where we want to push
            let before_node = self.get_node_at(idx - 1);

            if let Some(temp_val) = before_node {
                let before_node_ptr = temp_val.as_ptr();

                unsafe {
                    let new_node = Box::new(LinkedList {
                        value: Some(Box::new(value)),
                        next: (*before_node_ptr).next,
                    });

                    // now we set the value
                    (*before_node_ptr).next = Some(Box::leak(new_node).into());
                }
            } else {
                // we cannot push here
                return Err(());
            }
        }
        Ok(())
    }

    /// Gets an element in the linked list at this index.
    ///
    /// # Params
    /// - `idx` - The index in the list to get the value from.
    ///
    /// # Returns
    /// - `Some(StoreType)` if the value could be found, `None` otherwise.
    pub fn get(&self, idx: usize) -> Option<&StoreType> {
        let node = self.get_node_at(idx);

        match node {
            Some(temp_val) => {
                // SAFETY: temp_val is always valid
                unsafe { temp_val.as_ref().value.as_deref() }
            }
            None => None,
        }
    }
}

impl<StoreType> Drop for LinkedList<StoreType> {
    fn drop(&mut self) {
        // since this is recursive, we will just drop our own stuff
        if self.next.is_some() {
            // we will have to drop NonNulls which are allocated on the heap
            unsafe {
                let next_node_ptr = self.next.unwrap_unchecked().as_ptr();

                // we can drop the next node, calling its drop() function and continuing the loop
                drop(Box::from_raw(next_node_ptr));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_list_operations() {
        let mut list = LinkedList::<i32>::new();

        list.push_front(4);
        assert_eq!(4, *list.get(0).unwrap());

        list.push_front(5);
        assert_eq!(5, *list.get(0).unwrap());
        assert_eq!(4, *list.get(1).unwrap());

        list.remove(0)
            .expect("Removing this element should not cause an error");
        assert_eq!(4, *list.get(0).unwrap());

        list.push_back(100);
        assert_eq!(4, *list.get(0).unwrap());
        assert_eq!(100, *list.get(1).unwrap());
    }

    #[test]
    fn test_larger_push_back() {
        let mut list = LinkedList::<i32>::new();

        list.push_back(100);
        list.push_back(101);
        list.push_back(102);
        list.push_back(103);
        list.push_back(104);
        assert_eq!(100, *list.get(0).unwrap());
        assert_eq!(101, *list.get(1).unwrap());
        assert_eq!(102, *list.get(2).unwrap());
        assert_eq!(103, *list.get(3).unwrap());
        assert_eq!(104, *list.get(4).unwrap());
    }

    #[test]
    fn test_add_at() {
        let mut list = LinkedList::<i32>::new();

        list.add_at(1, 0).unwrap();
        list.add_at(3, 1).unwrap();
        list.add_at(4, 2).unwrap();
        list.add_at(0, 0).unwrap();
        list.add_at(2, 2).unwrap();

        assert_eq!(0, *list.get(0).unwrap());
        assert_eq!(1, *list.get(1).unwrap());
        assert_eq!(2, *list.get(2).unwrap());
        assert_eq!(3, *list.get(3).unwrap());
        assert_eq!(4, *list.get(4).unwrap());
    }

    #[test]
    fn test_remove() {
        let mut list = LinkedList::<i32>::new();

        list.push_back(0);
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);

        list.remove(0).unwrap();
        list.remove(2).unwrap();
        list.remove(2).unwrap();
        list.remove(2)
            .expect_err("Expected an error when deleting a value that doesn't exist");

        assert_eq!(1, *list.get(0).unwrap());
        assert_eq!(2, *list.get(1).unwrap());
    }

    #[test]
    fn test_push_front() {
        let mut list = LinkedList::<i32>::new();

        list.push_front(4);
        list.push_front(3);
        list.push_front(2);
        list.push_front(1);
        list.push_front(0);

        assert_eq!(0, *list.get(0).unwrap());
        assert_eq!(1, *list.get(1).unwrap());
        assert_eq!(2, *list.get(2).unwrap());
        assert_eq!(3, *list.get(3).unwrap());
        assert_eq!(4, *list.get(4).unwrap());
    }

    #[test]
    fn test_push_back() {
        let mut list = LinkedList::<i32>::new();

        list.push_back(0);
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);

        assert_eq!(0, *list.get(0).unwrap());
        assert_eq!(1, *list.get(1).unwrap());
        assert_eq!(2, *list.get(2).unwrap());
        assert_eq!(3, *list.get(3).unwrap());
        assert_eq!(4, *list.get(4).unwrap());
    }

    #[test]
    fn test_get() {
        let mut list = LinkedList::<i32>::new();

        let mut result = list.get(0);
        if result.is_some() {
            assert!(false); // fail
        }
        list.push_back(0);
        assert_eq!(0, *list.get(0).unwrap());
        list.push_back(1);
        assert_eq!(1, *list.get(1).unwrap());
        list.push_back(2);
        assert_eq!(2, *list.get(2).unwrap());
        list.push_back(3);
        assert_eq!(3, *list.get(3).unwrap());
        list.push_back(4);
        assert_eq!(4, *list.get(4).unwrap());
        result = list.get(5);
        if result.is_some() {
            assert!(false); // fail
        }
    }
}
