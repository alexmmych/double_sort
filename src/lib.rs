/*!
Double sort works by separating the elements into pairs of two called nodes, this allows
to order the two elements within the node extremely fast since it's a 1:1 memory swap.

Grouping elements by nodes allows a simple loop statement to order the whole vector in n/2 - 1 reads maximum by utilizing BinaryHeaps

# Usage

```toml
[dependencies]
double_sort = "1.1.1"
```
*/

#[derive(PartialEq,PartialOrd,Eq,Debug,Clone, Copy,Ord)]
struct Node<T>(T,Option<T>); 

impl<T: PartialOrd + Copy> Node<T> {
    //Rearranges the node in the correct order
    fn order(&mut self) {
        if let Some(_number) = self.1 {
            switch(&mut self.0,self.1.as_mut().unwrap());
        }
    }

    //Informs if there is None present in the structure
    fn none_present(&self) -> bool {
        if self.1 == None {
            true
        } else {
            false
        }
    }

    fn contains(&self,element:T) -> bool {
        if element == self.0 {
            true
        } else {
            false
        }
    }

    fn change_vector(&self, array: &mut Vec<T>, index: usize) {
        array[index] = self.0;
        if let Some(number) = self.1 {
            array[index+1] = number;
        }
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

///Sorts a vector of elements by pairing them and ordering them by the lowest number, then exchanging elements with neighbours until sorted.
/// # Example 

///```rust
///use double_sort::double_sort;

///fn main() {
///    let mut vector = vec![48,23,78,67,89,22,33,44];
///  
///    double_sort(&mut vector);
/// 
///    assert_eq!(vector,[22,23,33,44,48,67,78,89]);
///}
///```
pub fn double_sort<T: Copy + Ord>(array: &mut Vec<T>) {


    if array.len() <= 2 {

        if array.len() == 1 {
            return;
        }

        let mut node = Node(array[0],Some(array[1]));
        node.order();

        node.change_vector(array, 0);
        
        return;
    }

    //Mutable values used to control the while loop
    let mut _counter = 0; //Amount of times the loop ran for


    let mut vector = Vec::new();

    let iter = array.chunks_exact(2);
    let mut node: Node<T>;
    let temp_node: Option<Node<T>>;

    if !iter.remainder().is_empty() {
        temp_node = Some(Node(iter.remainder()[0],None));

        temp_node.unwrap().order();
    } else {
        temp_node = None;
    }

    for chunk in iter {
        node = Node(chunk[0],Some(chunk[1]));

        node.order();

        vector.push(node);
    }

    if let Some(_element) = temp_node {
        vector.push(temp_node.unwrap());
    }
    

    let mut reference_vec = Vec::new();
    let mut temp_vec = Vec::new();

    for node in &vector {
        temp_vec.push(node.0);
    }

    double_heap_sort(&mut temp_vec);

    for reference in &temp_vec {
        let left_node = *vector.iter().find(|x| x.contains(*reference) == true).unwrap();

        reference_vec.push(left_node);
    }

    vector = reference_vec;


    let mut index = 0;

    //Final sort of the values by comparing left and right values of neighbouring nodes
    loop {
        let mut left = vector[_counter];

        if _counter == vector.len() - 1 {
            left.change_vector(array, index);
            break;
        }

        let mut right = vector[_counter+1];

        let switched: bool; //Checks whether anything was changed

        if let Some(_number) = left.1 {
            switched = switch(left.1.as_mut().unwrap(),&mut right.0);
        } else {
            switched = switch(&mut left.0,&mut right.0);
        }

        let mut end_left = vector[vector.len() - 2];
        let mut end_right = vector[vector.len() - 1];

        if let Some(_number) = end_left.1 {
            switch(end_left.1.as_mut().unwrap() ,&mut end_right.0);
        } else {
            switch(&mut end_left.0,&mut end_right.0);
        }

        if !switched {
            left.change_vector(array, index);
            vector[_counter] = left;

            _counter += 1;

            if left.none_present() {
                index += 1;
            } else {
                index += 2;
            }

            if right.none_present() {
                right.change_vector(array, index);
                vector[_counter] = right;

                if _counter == vector.len() - 1 {
                    break;
                }

                index += 1;
                _counter += 1;

                if _counter == vector.len() - 2 {
                    let left = vector[_counter];

                    left.change_vector(array, index);

                    break;
                }
                
            }
            continue;
        }

        left.order();
        right.order();

        vector[_counter] = left;
        vector[_counter+1] = right;

        if _counter == vector.len() - 1 {
            left.change_vector(array, index);

            if left.none_present() {
                index += 1;
            } else {
                index += 2;
            }

            right.change_vector(array, index); //Index out of bounds on random_sort
            break;
        }

        //Increment _counter
        _counter += 1;


        //Everything is pushed back into the heap so nothing is lost.
        left.change_vector(array, index);

        if left.none_present() {
            index += 1;
        } else {
            index += 2;
        }

        let mut temporary_vec = Vec::new();
        let mut reference_vec = Vec::new();

        for node in &vector {
            temporary_vec.push(node.0);
        }
    
        double_heap_sort(&mut temporary_vec);

        if temporary_vec != temp_vec {
            for reference in &temporary_vec {
                let left_node = *vector.iter().find(|x| x.contains(*reference) == true).unwrap();
        
                reference_vec.push(left_node);
            }
        
            vector = reference_vec;
        }

    }

}



///Works in the same way that [double_sort] does but utilizes [BinaryHeaps](std::collections::BinaryHeap). This provides a faster but less logarithmic sort since
/// it only shaves half [heap.pop()](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html#method.pop) runs.
/// # Note
/// This was the previous [double_sort] in 1.0.0
/// 
///# Example 
/// 
///```rust
///use double_sort::double_heap_sort;
/// 
///fn main() {
///    let mut vector = vec![48,23,78,67,89,22,33,44];
///  
///    double_heap_sort(&mut vector);
/// 
///    assert_eq!(vector,[22,23,33,44,48,67,78,89]);
///}
///```
/// 
/// # Debug Assertion Output
/// 
/// This output will only be shown if you have debug enabled
/// 
///```text
///Time creating nodes: 4.09µs
///Time looping: 4.33µs
///Total function time: 14.37µs
///Total reads done: 3
///Total number of memory switches: 2
///
pub fn double_heap_sort<T: Copy + Ord>(array: &mut Vec<T>) {

    use std::collections::BinaryHeap;
    use std::cmp::Reverse;
    
    if array.len() <= 2 {
        let mut node = Node(array[0],array.get(1).cloned());
        node.order();

        node.change_vector(array, 0);
        
        return;
    }

    //BinaryHeap is used due to it's efficient ordering capabilities.
    let mut heap = BinaryHeap::new();

    //Mutable values used to control the while loop
    let mut _counter = 0; //Amount of times the loop ran for

    let iter = array.chunks_exact(2);
    let mut node: Node<T>;

    if !iter.remainder().is_empty() {
        node = Node(iter.remainder()[0],None);

        node.order();
        heap.push(Reverse(node));
    }

    for chunk in iter {
        node = Node(chunk[0],Some(chunk[1]));

        node.order();
        heap.push(Reverse(node));
    }

    let mut index = 0;

    //Final sort of the values by comparing left and right values of neighbouring nodes
    loop {

        if heap.is_empty() {
            break;
        }

        let mut left = heap.pop().unwrap().0;

        if heap.is_empty() {
            left.change_vector(array, index);
            break;
        }

        let mut right = heap.pop().unwrap().0;

        let switched: bool; //Checks whether anything was changed

        if let Some(_number) = left.1 {
            switched = switch(left.1.as_mut().unwrap(),&mut right.0);
        } else {
            switched = switch(&mut left.0,&mut right.0);
        }

        if !switched {
            left.change_vector(array, index);

            //Increment the times where read did nothing
            _counter += 1;

            if left.none_present() {
                index += 1;
            } else {
                index += 2;
            }

            if right.none_present() {
                right.change_vector(array, index);

                index += 1;

                if heap.len() == 1 {
                    let left = heap.pop().unwrap().0;

                    left.change_vector(array, index);

                    break;
                }
                
            } else {
                heap.push(Reverse(right));
            }

            continue;
        }

        left.order();
        right.order();

        //Increment _counter
        _counter += 1;

        if heap.is_empty() {
            left.change_vector(array, index);

            if left.none_present() {
                index += 1;
            } else {
                index += 2;
            }

            right.change_vector(array, index); //Index out of bounds on random_sort
            break;
        }

        //Everything is pushed back into the heap so nothing is lost.
        left.change_vector(array, index);
        
        if left.none_present() {
            index += 1;
        } else {
            index += 2;
        }

        heap.push(Reverse(right));

    }
}

