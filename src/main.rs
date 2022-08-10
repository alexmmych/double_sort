//The structure implements an Option in the case that
//the total amount of numbers is uneven.

//Tuple sort works by separating the numbers into pairs of two called nodes, this allows
//to order the two numbers within the node extremely fast since it's a 1:1 and memory swap

//After the vector of numbers has been given to the function, it will divide them into nodes using chunks
//and it will proceed to sort them firstly by using a min BinaryHeap which will pop the in ascending order automatically
//and then it compares the right most number against the neighbour's left most number to see which one is smaller
//and changes them accordingly until fully solved.

use std::cmp::Ordering;
use rudac::heap::FibonacciHeap;


#[derive(PartialEq, PartialOrd,Eq,Debug)]
struct Node(u32,Option<u32>); //Option<u32> is given in the case that the total amount of numbers is uneven.

impl Node {
    //Rearranges the node in the correct order
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

fn switch<T: PartialOrd>(left: &mut T,right: &mut T) -> bool {
    if left > right {
        std::mem::swap(left,right);
        return true;
    } else {
    false
    }
}

//Sorts a vector of numbers
fn tuple_sort(list: &mut Vec<u32>) {
    //Panics if the vector is empty
    if list.is_empty() {
        panic!("tuple_sort() called with empty vector: {:?}", list.as_ptr());
    }

    //BinaryHeap is used due to it's efficient ordering capabilities.
    let mut heap = FibonacciHeap::init_min();

    //Creates nodes from chunks of the vector
    for chunk in list.chunks(2) {
        let mut node: Node;

        if chunk.len() == 2 {
        node = Node(chunk[0],Some(chunk[1]));
        } else {
            node = Node(chunk[0],None);
        }

        node.order();

        heap.push(node);
    }

    //Clears the list so it can be given the sorted slices
    list.clear();

    //Important check to instantly return ordered vector if it only contains 1 or 2 elements.
    if heap.size() == 1 {
        list.append(&mut heap.pop().unwrap().slices());
        return;
    }

    //Mutable values used to control the while loop
    let mut sorted = false;
    let mut counter = 0;
    let mut nothing = 0;
    let mut total = 1;
    
    //Final sort of the values by comparing left and right values of neighbouring nodes
    while sorted == false  {
        let mut temp_heap = FibonacciHeap::init_min();

        //Temporary heap pushes already sorted nodes into itself.
        for _i in 0..counter {
            temp_heap.push(heap.pop().unwrap());
        }

        let mut left = heap.pop().unwrap();
        let mut right = heap.pop().unwrap();

        let mut switched = false; //Checks whether anything was changed

        if let Some(_number) = left.1 {
            switched = switch(left.1.as_mut().unwrap(),&mut right.0);
        } else {
            switched = switch(&mut left.0,&mut right.0);
        }

        if !switched {
            //Increment the times where read did nothing
            nothing += 1;
        }

        left.order();
        right.order();

        //Everything is pushed back into the heap so nothing is lost.
        heap.push(left);
        heap.push(right);

        heap = FibonacciHeap::merge(heap,temp_heap);


        //Max amount of reads needed (n/2 - 1)
        if counter == heap.size() - 2 { //-2 because .len() doesn't count 0
            //Info dump
            println!("Total reads done: {}",total); //+1 because index 0 also counts as a read
            println!("Total number of memory switches: {}", total - nothing);

            //Closing boolean
            sorted = true;
        }

        //Increment counter
        counter += 1;
        total += 1;
    }

    //Pops the heap into the list in order
    for _i in 0..total{
        list.append(&mut heap.pop().unwrap().slices());
    }



}

fn main() {
    //The numbers to be ordered
    let mut numbers = vec![48,23,78,67,89,22,33,44];

    tuple_sort(&mut numbers);

    println!("Result: {:?}",numbers);
}
