use std::ptr::NonNull;

/// Linked List struct that can hold any type of value.
pub struct LinkedList<StoreType> {
    value: Option<Box<StoreType>>,
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
    pub fn remove(&mut self, idx: usize) -> bool {
        let mut cur_node; // our current value

        if idx == 0 {
            // remove the head of the list
            if let Some(temp_val) = self.next {
                cur_node = temp_val.as_ptr();
            } else {
                return false; // expected a value but got none
            }

            // SAFETY: cur_node is always Some value
            unsafe {
                // set to what the cur_node has as its next node.
                // Could be Some or None
                self.next = (*cur_node).next;
            }
        } else {
            // remove some node in the middle/end of the list

            if let Some(temp_val) = self.next {
                cur_node = temp_val.as_ptr();
            } else {
                return false; // fail, empty list
            }

            let mut cur_idx = 0;
            // keep going until we are at the value right before
            while cur_idx < idx {
                // SAFETY: cur_node is always Some value
                unsafe {
                    if let Some(temp_val) = (*cur_node).next {
                        cur_node = temp_val.as_ptr();
                    } else {
                        return false; // fail, expected a value to be here and there wasn't
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
                    return false; // expected the next value to exist, but it doesn't
                }

                if (*node_to_remove).next.is_some() {
                    // we have a value to point to
                    (*cur_node).next = (*node_to_remove).next;
                }
                // drop the node to remove now
                drop(Box::from_raw(node_to_remove));
            }
        }

        true
    }

    /// Pushes a value at the beginning of the list.
    /// Sets this value as the new head.
    pub fn push_front(&mut self, value: StoreType) -> bool {
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

        true
    }

    /// Pushes a value at the end of the list.
    pub fn push_back(&mut self, value: StoreType) -> bool {
        if self.next.is_none() {
            // empty list, push to the front

            let new_node = Box::new(LinkedList {
                value: Some(Box::new(value)),
                next: None,
            });
            self.next = Some(Box::leak(new_node).into());
        } else {
            let mut cur_node; // our current value

            if let Some(temp_val) = self.next {
                cur_node = temp_val.as_ptr();
            } else {
                return false; // fail, empty list
            }

            // SAFETY: cur_node is always Some value
            unsafe {
                // keep going until we are at the last node
                while (*cur_node).next.is_some() {
                    if let Some(temp_val) = (*cur_node).next {
                        cur_node = temp_val.as_ptr();
                    } else {
                        return false; // fail, expected a value to be here and there wasn't
                    }
                }

                // allocate on the heap
                let new_node = Box::new(LinkedList {
                    value: Some(Box::new(value)),
                    next: None,
                });
                // this is the new tail of the list
                (*cur_node).next = Some(Box::leak(new_node).into());
            }
        }

        true
    }

    pub fn get(&self, idx: usize) -> Option<&StoreType> {
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

        match cur_node {
            Some(temp_val) => {
                // SAFETY: temp_val is always valid
                unsafe { temp_val.as_ref().value.as_deref() }
            }
            None => None,
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

        list.remove(0);
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
}
