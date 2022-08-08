//The structure implements an Option in the case that
//the total amount of numbers is uneven.

//Tuple sort works by separating the numbers into pairs of two called nodes, this allows
//to order the two numbers within the node extremely fast since it's a 1:1 and memory swap

//After the vector of numbers has been given to the function, it will divide them into nodes using chunks
//and it will proceed to sort them firstly by using a min BinaryHeap which will pop the in ascending order automatically
//and then it compares the right most number against the neighbour's left most number to see which one is smaller
//and changes them accordingly until fully solved.

use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::cmp::Ordering;

#[derive(PartialEq, PartialOrd,Eq,Debug)]
struct Node(u32,Option<u32>); //Option<u32> is given in the case that the total amount of numbers is uneven.

impl Node {
    //Rearranges the node to be ordered.
    fn order(&mut self) {
        if let Some(_number) = self.1 {
            switch(&mut self.0,self.1.as_mut().unwrap());
        }
    }

    //Returns the node in form of a vector slice
    fn slices(&self) -> Vec<u32> {
        let mut vector = Vec::new();
        vector.push(self.0);
    
        if let Some(number) = self.1 {
            vector.push(number);
        }
    
        vector
    }
}

//Implementation for order so BinaryHeap only considers leftmost value
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

fn switch<T: PartialOrd>(left: &mut T,right: &mut T) {
    if left > right {
        std::mem::swap(left,right);
    }
}

//Sorts a vector of numbers
fn tuple_sort(list: &mut [u32]) -> Vec<u32> {

    //Panics if the vector is empty
    if list.is_empty() {
        panic!("tuple_sort() called with an empty vector.");
    }

    let mut heap = BinaryHeap::new();
    let mut vec = Vec::new();
    let mut result = Vec::new(); //The vector to be given at end.

    //Creates nodes from chunks of the vector
    for chunk in list.chunks(2) {
        let mut node: Node;

        if chunk.len() == 2 {
        node = Node(chunk[0],Some(chunk[1]));
        } else {
            node = Node(chunk[0],None);
        }

        node.order();

        heap.push(Reverse(node));
    }

    //Important check to instantly return ordered vector if it only contains 1 or 2 elements.
    if heap.len() == 1 {
        result.append(&mut heap.pop().unwrap().0.slices());
        return result;
    }


    //Mutable values used to control the while loop
    let mut sorted = false;
    let mut counter = 0;
    let mut reset = 0;

    //Moves the sorted nodes into the vector
    while !heap.is_empty() {
        vec.push(heap.pop().unwrap().0);
    }
    
    //Info on how many times the while loop ran, including resets.
    let mut total = 0;

    //Final sort of the values by comparing right and left values of neighbouring nodes
    while sorted == false  {
        
        //Guard if if the stored value is None.
        if let Some(_number) = vec[counter].1 {
            let (left,right) = vec.split_at_mut(counter+1);

            switch(left[counter].1.as_mut().unwrap(), &mut right[0].0);

            left[counter].order();
            right[0].order();
        }

        //Increment counter
        counter += 1;

        //Info
        total += 1;

        //Reset if to avoid accessing out of bounds
        if counter == vec.len() - 1 { // -1 because the if above adds +1 to the counter

            if reset == 1 {
                //Info dump
                println!("Total reads done: {}",total);
                //Closing boolean
                sorted = true;
            }

            //Resets counter and increments the reset variable
            counter = 0;
            reset += 1;
        }
    }
    
    for node in vec {
        result.append(&mut node.slices());
    } 


    result
}

fn main() {
    //The numbers to be ordered
    let mut numbers = vec![75,23,5,6,7234,734,7345,788,56];


    let result = tuple_sort(&mut numbers);
    println!("Result: {:?}",result);
}
