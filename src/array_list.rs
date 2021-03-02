use core::ptr::{self};

pub struct ArrayList<T> {
    items: Vec<T>,
    size: usize
}

impl<T> ArrayList<T> {
    // Initializes an `ArrayList`
    pub fn new() -> ArrayList<T> {
        Default::default()
    }

    // Returns the length
    pub fn len(&self) -> usize {
        self.size
    }

    //  Returns the value at a specific location
    pub fn get(&self, i: usize) -> Option<&T> {
        self.items.get(i)
    }

    // Adds a new item to the end
    pub fn add(&mut self, item: T) {
        if self.size == self.items.len() {
            self.resize();
        }
        self.items.push(item);
        self.size += 1;
    }

    // Removes an item
    pub fn remove(&mut self, index: usize) -> Result<T, String> {
        if index < self.size {
            unsafe {
                // infallible
                let ret;
                {
                    // the place we are taking from.
                    let ptr = self.items.as_mut_ptr().add(index);
                    // copy it out, unsafely having a copy of the value on
                    // the stack and in the vector at the same time.
                    ret = ptr::read(ptr);
    
                    // Shift everything down to fill in that spot.
                    ptr::copy(ptr.offset(1), ptr, self.size - index - 1);
                }
                self.size -= 1;
                Ok(ret)
            }
        } else {
            Err(String::from("Index does not exist."))
        }
    }

    pub fn set(&mut self, index: usize, item: T) -> Result<(), String> {
        if index < self.size {
            self.items[index] = item;
            Ok(())
        } else {
            Err(String::from("Index does not exist."))
        }
    }

    fn resize(&mut self) {
        let new_size: usize;
        if self.size == 0 {
            new_size = 1
        } else {
            new_size = self.size * 2
        }
        let mut new_items = Vec::with_capacity(new_size);
        
        for item in &self.items {
            new_items.push(item);
        }
    }
}

impl<T> Default for ArrayList<T> {
    fn default() -> ArrayList<T> {
        ArrayList { items: Vec::with_capacity(0), size: 0 }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_list_init() {
        let list: ArrayList<usize> = ArrayList::new();
        assert_eq!(list.len(), 0);
    }
    
    #[test]
    fn array_list_add() {
        let mut list: ArrayList<usize> = ArrayList::new();
        list.add(22);
        assert_eq!(list.len(), 1);
        assert!(list.get(0).is_some());
        assert_eq!(list.get(0).unwrap(), &22);
    }

    #[test]
    fn array_list_add_2() {
        let mut list: ArrayList<usize> = ArrayList::new();
        list.add(22);
        list.add(24);
        assert_eq!(list.len(), 2);
        assert!(list.get(0).is_some());
        assert_eq!(list.get(0).unwrap(), &22);
        assert_eq!(list.get(1).unwrap(), &24);
    }

    #[test]
    fn array_list_set() {
        let mut list: ArrayList<usize> = ArrayList::new();
        list.add(22);
        assert!(list.set(0, 24).is_ok());
        assert_eq!(list.get(0).unwrap(), &24);
    }

    #[test]
    fn array_list_remove() {
        let mut list: ArrayList<usize> = ArrayList::new();
        list.add(22);
        list.add(32);
        list.add(42);
        let result = list.remove(1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 32);
        println!("{:?}", list.items);
        assert_eq!(list.len(), 2);
    }
}
