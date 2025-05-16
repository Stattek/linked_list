use std::ptr::NonNull;

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

            // SAFETY: cur_node is always Some
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
                // SAFETY: cur_val is always Some value
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
            // SAFETY: cur_val is always Some value
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
    pub fn push_left(&mut self, value: StoreType) {
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
            unsafe {
                // new node should point to the current head
                new_node_ptr.as_mut().next = self.next;
            }
            // head is now the new pointer
            self.next = Some(new_node_ptr);
        }
    }

    pub fn get(&self, idx: usize) -> Option<&StoreType> {
        let mut cur_val = &self.next;
        let mut cur_idx = 0;

        // keep going until we have our value or we reach a none
        while cur_idx < idx && cur_val.is_some() {
            // SAFETY: cur_val is always Some value
            unsafe {
                cur_val = &cur_val.unwrap_unchecked().as_ref().next;
            }
            cur_idx += 1;
        }

        match cur_val {
            Some(temp_val) => unsafe { temp_val.as_ref().value.as_deref() },
            None => None,
        }
    }
}
