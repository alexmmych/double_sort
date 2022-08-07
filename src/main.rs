//The structure implements an Option in the case that
//the total amount of numbers is uneven.

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

fn tuple_sort(list: &mut [u32]) -> Vec<Vec<u32>> {
    let mut sorted_vector = Vec::new();

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

    let mut sorted = false;
    let mut counter = 0;
    let mut reset = 0;
    let mut total = 0;

    while sorted == false  {

        if let Some(number) = sorted_vector[counter].1 {

            if number > sorted_vector[counter+1].0 {
                let temp = sorted_vector[counter+1].0;

                sorted_vector[counter+1].0 = number;
                *sorted_vector[counter].1.as_mut().unwrap() = temp;

                sorted_vector[counter].order();
                sorted_vector[counter+1].order();
            } 
        }

        counter += 1;
        total += 1;

        if counter == sorted_vector.len() - 1 {

            if reset >= sorted_vector.len() {
                println!("Total reads done: {}",total);
                println!("Total resets done: {}",reset);
                sorted = true;
            }

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
