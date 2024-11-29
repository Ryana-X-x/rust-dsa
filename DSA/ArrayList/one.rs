//  ! ArrayList

/*  
? Vec is a growable array-like collection that allows storing elements of the same type
? Unlike arrays, a vec an dynamically grow or shrink if needed 
? It is stored on the heap, so that size does not need to be known at compile time
*/

/*  
* export default class ArrayList<T> {
*     public length : number ;
*     constructor() {} 
*     prepend(item: T) : void {}
*     insertAt(item: T, idx: number): void {}
*     append(item: T): void {}
*     remove(item: T): T | undefined() {}
*     get(idx: number): T | undefined {}
*     removeAt(idx: number): T | undefined {}
*}
*/

use std::fmt::Debug; 
type Array<T> = Vec<T> ;

#[derive(debug)]
struct ArrayList<T {
    pub length: usize,
    inner: Array<T, //*  The underlying storage for {Vec<T>}
}

impl<T: Debug + Default + Clone + PartailEq> ArrayList<T> {
    pub fn new() -> ArrayList<T> {
        ArrayList {
            length: 0, 
            inner: vec![T::default(); 5],
        }
    }

    // ? Dynamically Growing The Array -> Unlike Java or Python lists, Rust's Vec doesn't automatically grow while ensuring all safety guarantees in custom implmentation  
    // ? When the Array is full, grow_inner doubles it's size and copies the elements into a new storage vector.

    fn grow_inner(&mut self) {
        // Step 1: Reference the current storage (inner array)
        let prev = &self.inner;

        // Step 2: Create a new storage array, twice the size of the current one
        let mut new = vec![T::default(); prev.len() * 2] ;

        // Step 3: Copy all existing elements from the old array to the new one
        for i in 0..prev.len() {
            new[i] = prev[i].clone() ;
        }

        // Step 4 Replace the old array with the new larger array
        self.inner = new ;
    }
    
    // ? It ensures the internal storage {inne} has enough capacity to accomodate the new element by resizing when necessary 
    pub fn append(&mut self, item: T) {
        // Step 1: Check if current storage is full or not
        if self.inner.len() == self.len {
            self.grow_inner() ;
        }

        // Step 2: Add the value to the cureent end of the list
        self.inner[self.length] = item ;
        self.length += 1;
    }

    // ! Visualization 
    /*
    * inner : [0,0,0,0,0]   // initialized with default values
    * length : 0
    ? After appending 1 element
    * inner: [10, 0, 0, 0, 0]
    * length: 1
    ? After append when array is full 
    * inner : [10, 2, 5, 23, 4]
    * length : 5
    ? We will call grow_inner
    * inner : [10, 2, 5, 23, 4, 0, 0, 0, 0, 0]
    * length : [10]
    ? Now we will append at the end of the 
    * inner[10, 2, 5, 23, 4, 6, 0, 0, 0, 0]
    * length : [10]
     */


    //? Popping an elemnet
    pub fn pop(&mut self) -> Option<T> {
        // Step 1: If the list is empty, if so, return None
        if self.length == 0 {
            return None ;
        }

        // Step 2: Calculate the index of the last valid element
        let tail = self.length - 1;

        // Clone The value at the last index to return later 
        let item = self.inner[tail].clone ;

        // Step 4: Clear the last slot by replacing it with the default value of T
        self.inner[tail] = T::default() ;

        // Step 5: Reduce the logical size of the list to exclude the last element
        self.length = tail ; 

        // Return the cloned item wrapped in Some
        Some(item)
    }

    // ? Insert at index 
    pub fn insert_at(&mut self, index: usize, item: T) {
        // Step 1: Check if the array is full or not 
        if self.inner.len() == self.length {
            self.grow_inner() ;
        }

        // Step 2: Shift elements to the right to make space for the new elements
        for i in ((index + 1)..=self.length).rev() {
            self.inner[i] = self.inner[i - 1].clone() ;
        }

        // Step 3: Insert the new item at the specified index
        self.inner[index] = item ;

        // Step 4: Increment the length to account for the new element
        self.length += 1 ;
        
    }     

    // ? Remove at Index
    pub fn remove_at(&mut self, index: usize) -> Option<T> {
        // Step 1: Check if the index is valid 
        if index < self.length {
            // Step 2: Retrieve the item at the given index to return later 
            let item = self.inner[index].clone() ;

            for i in index..self.length -1 {
                self.inner[i] = self.inner[i + 1].clone() ;
            }

            // Step 3: Clear the non used item space 
            self.inner[self.length - 1] = T::default() ;

            // Updating the length to show the removal
            self.length =- 1 ;

            Some(item) ;
        }else {
            None
        }
}