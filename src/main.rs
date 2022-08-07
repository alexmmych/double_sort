//The structure implements an Option in the case that
//the total amount of numbers is uneven.

//Tuple sort works by separating the numbers into pairs of two called nodes, this allows
//to order the two numbers within the node extremely fast since it's a 1:1 comparison

//After the given vector of numbers has been given to the function, it will divide them into nodes using chunks
//and it will proceed to sort them firstly by sorting the lowest numbers and setting them in position
//and then comparing the right most numbers against the neighbour's left most number to see which one is smaller
//and changes them accordingly until fully solved.

#[derive(Copy, Clone)]
struct Node(u32,Option<u32>);

impl Node {
    //Rearranges the node to be ordered.
    fn order(&mut self) {
        if let Some(number) = self.1 {
            if number < self.0 {
                let temp = self.0;
                self.0 = number;
                *self.1.as_mut().unwrap() = temp;
            }
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

/*
fn sort_loop(left: &u32, right: &u32) {
    
}
*/

//Sorts a vector of numbers
fn tuple_sort(list: &mut [u32]) -> Vec<Vec<u32>> {
    let mut sorted_vector = Vec::new();

    //Creates nodes from chunks of the vector
    for chunk in list.chunks(2) {
        let mut node: Node;

        if chunk.len() == 2 {
        node = Node(chunk[0],Some(chunk[1]));
        } else {
            node = Node(chunk[0],None);
        }

        node.order();

        sorted_vector.push(node);
    }

    //Mutable values used to control the while loop
    let mut sorted = false;
    let mut counter = 0;
    let mut reset = 0;

    //Initial ordering of the nodes by their leftmost values
    while sorted == false {
        if sorted_vector[counter].0 > sorted_vector[counter+1].0 {
            let temp = sorted_vector[counter];
            sorted_vector[counter] = sorted_vector[counter+1];
            sorted_vector[counter+1] = temp;
        }

        //Increment counter
        counter += 1;
        
        //Reset if to avoid accessing out of bounds
        if counter == sorted_vector.len() - 1 { // -1 because the if above adds +1 to the counter
        
            if reset >= sorted_vector.len() {
                //Closing boolean
                counter = 0;
                reset = 0;
                sorted = true;
            }
        
            //Resets counter and increments the reset variable
            counter = 0;
            reset += 1;
        }
    }

    //Reset of the previous values
    sorted = false;

    //Info on how many times the while loop ran, including resets.
    let mut total = 0;

    //Final sort of the values by comparing right and left values of neighbouring nodes
    while sorted == false  {
        
        //Guard if if the stored value is None.
        if let Some(number) = sorted_vector[counter].1 {

            if number > sorted_vector[counter+1].0 {
                let temp = sorted_vector[counter+1].0;

                sorted_vector[counter+1].0 = number;
                *sorted_vector[counter].1.as_mut().unwrap() = temp;

                sorted_vector[counter].order();
                sorted_vector[counter+1].order();
            } 
        }

        //Increment counter
        counter += 1;

        //Info
        total += 1;

        //Reset if to avoid accessing out of bounds
        if counter == sorted_vector.len() - 1 { // -1 because the if above adds +1 to the counter

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

    let mut result = Vec::new();

    for node in sorted_vector {
        result.push(node.slices());
    }

    result

}

fn main() {
    //The numbers to be ordered
    let mut numbers = vec![43,47,87,54,65,34,567,79,2];


    let result = tuple_sort(&mut numbers);

    println!("{:?}",result);
}
