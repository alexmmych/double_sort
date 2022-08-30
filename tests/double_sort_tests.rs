use double_sort::double_sort;

#[cfg(test)]
mod tests {

    use super::*;
    use rand::thread_rng;
    use rand::seq::SliceRandom; 

    #[test]
    fn random_sort() { //Runs a randomized big vector of elements as a test

        let mut vector: Vec<u32> = (0..1000).collect();

        let sorted_vec = vector.to_vec();

        vector.shuffle(&mut thread_rng());

        double_sort(&mut vector);

        assert_eq!(vector,sorted_vec,"Example vector was not sorted properly");
    }

    #[test]
    fn num_sort() { //Checks if the provided vector is sorted
        
        let mut vector = vec![48,23,78,67,89,22,33,44];

        double_sort(&mut vector);

        assert_eq!(vector,[22,23,33,44,48,67,78,89],"Example vector was not sorted properly");
    }

    #[test]
    fn uneven_sort() { //Checks if the function can sort an uneven amount of elements
        let mut vector = vec![42,23,5,6,12];

        double_sort(&mut vector);

        assert_eq!(vector,[5,6,12,23,42],"Uneven amount of elements were not sorted properly");
    }

    #[test]
    fn char_sort() { //Checks if characters get sorted alphabetically

        let mut vector = vec!['q','w','e','r','t','y','u','i','o','p','a','s','d','f','g','h','j','k','l','z','x','c','v','b','n','m']; //QWERTY layout
    

        double_sort(&mut vector);

        assert_eq!(vector, ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z']); //Alphabetic order
    }

    #[test]
    fn early_return() { //Checks if the function returns ordered when an early return occurs.

        let mut single_number_vector = vec![1];
        let mut two_number_vector = vec![2,1];

        double_sort(&mut single_number_vector);
        double_sort(&mut two_number_vector);

        assert_eq!(single_number_vector,[1]);
        assert_eq!(two_number_vector,[1,2]);

    }

}