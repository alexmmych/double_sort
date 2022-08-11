//The structure implements an Option in the case that
//the total amount of numbers is uneven.

//Tuple sort works by separating the numbers into pairs of two called nodes, this allows
//to order the two numbers within the node extremely fast since it's a 1:1 and memory swap

//After the vector of numbers has been given to the function, it will divide them into nodes using chunks
//and it will proceed to sort them firstly by using a min BinaryHeap which will pop the in ascending order automatically
//and then it compares the right most number against the neighbour's left most number to see which one is smaller
//and changes them accordingly until fully solved.

#[cfg(test)]
mod tests {
    #[cfg(debug_assertions)]
    use std::time::Instant;

    use super::*;

    #[test]
    fn num_sort() { //Checks if the provided vector is sorted
        #[cfg(debug_assertions)]
        let tracker = Instant::now();

        let mut vector = vec![48,23,78,67,89,22,33,44];
    
        tuple_sort::tuple_sort(&mut vector);

        assert_eq!(vector,[22,23,33,44,48,67,78,89],"Example vector was not sorted properly");

        #[cfg(debug_assertions)]
        println!("Elapsed time: {:.2?}",tracker.elapsed());
    }

    #[test]
    fn char_sort() { //Checks if characters get sorted alphabetically

        #[cfg(debug_assertions)]
        let tracker = Instant::now();

        let mut vector = vec!['q','w','e','r','t','y','u','i','o','p','a','s','d','f','g','h','j','k','l','z','x','c','v','b','n','m']; //QWERTY layout
    
        tuple_sort::tuple_sort(&mut vector);

        assert_eq!(vector, ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z']); //Alphabetic order

        #[cfg(debug_assertions)]
        println!("Elapsed time: {:.2?}",tracker.elapsed());
    }

    #[test]
    fn early_return() { //Checks if the function returns ordered when an early return occurs.
        #[cfg(debug_assertions)]
        let tracker = Instant::now();

        let mut single_number_vector = vec![1];
        let mut two_number_vector = vec![2,1];

        tuple_sort::tuple_sort(&mut single_number_vector);
        tuple_sort::tuple_sort(&mut two_number_vector);

        assert_eq!(single_number_vector,[1]);
        assert_eq!(two_number_vector,[1,2]);

        #[cfg(debug_assertions)]
        println!("Elapsed time: {:.2?}",tracker.elapsed());
    }

}

pub mod tuple_sort {

    use std::collections::BinaryHeap;
    use std::cmp::Reverse;
    use std::cmp::Ordering;

    #[derive(PartialEq,PartialOrd,Eq,Debug)]
    struct Node<T>(T,Option<T>); //Option<T> is given in the case that the total amount of numbers is uneven.

    impl<T: PartialOrd + Copy> Node<T> {
        //Rearranges the node in the correct order
        fn order(&mut self) {
            if let Some(_number) = self.1 {
                switch(&mut self.0,self.1.as_mut().unwrap());
            }
        }

        //Returns the node in form of a vector slice
        fn slices(&self) -> Vec<T> {
            let mut vector = Vec::new();
            vector.push(self.0);
        
            if let Some(number) = self.1 {
                vector.push(number);
            }
        
            vector
        }
    }
    
    //Checks if the left element is the smallest and if not then swaps with the right element so they are ordered from left to right.
    fn switch<T: PartialOrd>(left: &mut T,right: &mut T) -> bool {
        if left > right {
            std::mem::swap(left,right);
            return true;
        } else {
        false
        }
    }

    impl<T: PartialOrd + Eq + Ord> Ord for Node<T> {
        fn cmp(&self,other: &Self) -> Ordering {
            self.0.cmp(&other.0)
        }
    }

    //Sorts a vector of numbers by pairing them and ordering them by the lowest number, then exchanging numbers in order to sort.
    pub fn tuple_sort<T: Copy + Ord>(list: &mut Vec<T>) {
        //Panics if the vector is empty
        if list.is_empty() {
            panic!("tuple_sort() called with empty vector: {:?}", list.as_ptr());
        }

        //BinaryHeap is used due to it's efficient ordering capabilities.
        let mut heap = BinaryHeap::new();

        //Creates nodes from chunks of the vector
        for chunk in list.chunks(2) {
            let mut node: Node<T>;

            if chunk.len() == 2 {
                node = Node(chunk[0],Some(chunk[1]));
            } else {
                node = Node(chunk[0],None);
            }

            node.order();

            heap.push(Reverse(node));
        }

        //Clears the list so it can be given the sorted slices
        list.clear();

        //Important check to instantly return ordered vector if it only contains 1 or 2 elements.
        if heap.len() == 1 {
            list.append(&mut heap.pop().unwrap().0.slices());
            return;
        }

        //Mutable values used to control the while loop
        let mut sorted = false;
        let mut counter = 0;
        let mut nothing = 0; //Amount of times nothing was done on a read
        let mut total = 1;
        
        //Final sort of the values by comparing left and right values of neighbouring nodes
        while sorted == false  {
            let mut temp_heap = BinaryHeap::new();

            //Temporary heap pushes already sorted nodes into itself.
            for _i in 0..counter {
                temp_heap.push(heap.pop().unwrap());
            }

            let mut left = heap.pop().unwrap().0;
            let mut right = heap.pop().unwrap().0;

            let switched: bool; //Checks whether anything was changed

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
            heap.push(Reverse(left));
            heap.push(Reverse(right));

            heap.append(&mut temp_heap);


            //Max amount of reads needed (n/2 - 1)
            if counter == heap.len() - 2 { //-2 because .len() doesn't count 0

                //Info dump
                #[cfg(debug_assertions)]
                {
                    println!(""); //Whitespace
                    println!("Total reads done: {}",total);
                    println!("Total number of memory switches: {}", total - nothing);
                    println!(""); //Whitespace
                }

                //Closing boolean
                sorted = true;
            }

            //Increment counter
            counter += 1;
            total += 1;
        }

        //Pops the heap into the list in order
        for _i in 0..total{
            list.append(&mut heap.pop().unwrap().0.slices());
        }

    }

}