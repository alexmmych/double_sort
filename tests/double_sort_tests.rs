use double_sort::double_sort;
use double_sort::double_heap_sort;

#[cfg(test)]
mod tests {

    use super::*;
    use rand::Rng;

    #[test]
    fn random_sort() { //Runs a randomized big vector of elements as a test
        let mut rng = rand::thread_rng();

        let mut vector = Vec::new();

        for _i in 0..100 {
            vector.push(rng.gen_range(0..100));
        }

        let mut heap_vector = vector.to_vec();

        double_sort(&mut vector);
        double_heap_sort(&mut heap_vector);

        println!("Sorted Vector: {:?}",vector);
        println!("Sorted Vector: {:?}",heap_vector);
    }

    #[test]
    fn num_sort() { //Checks if the provided vector is sorted
        
        let mut vector = vec![48,23,78,67,89,22,33,44];
    
        let mut heap_vector = vector.to_vec();

        double_sort(&mut vector);
        double_heap_sort(&mut heap_vector);

        assert_eq!(vector,[22,23,33,44,48,67,78,89],"Example vector was not sorted properly");
        assert_eq!(heap_vector,[22,23,33,44,48,67,78,89],"Example vector was not sorted properly");
    }

    #[test]
    fn uneven_sort() { //Checks if the function can sort an uneven amount of elements
        let mut vector = vec![42,23,5,6,12];

        let mut heap_vector = vector.to_vec();

        double_sort(&mut vector);
        double_heap_sort(&mut heap_vector);

        assert_eq!(vector,[5,6,12,23,42],"Uneven amount of elements were not sorted properly");
        assert_eq!(heap_vector,[5,6,12,23,42],"Uneven amount of elements were not sorted properly");
    }

    #[test]
    fn char_sort() { //Checks if characters get sorted alphabetically

        let mut vector = vec!['q','w','e','r','t','y','u','i','o','p','a','s','d','f','g','h','j','k','l','z','x','c','v','b','n','m']; //QWERTY layout
    
        let mut heap_vector = vector.to_vec();

        double_sort(&mut vector);
        double_heap_sort(&mut heap_vector);

        assert_eq!(vector, ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z']); //Alphabetic order
        assert_eq!(heap_vector, ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z']);
    }

    #[test]
    fn early_return() { //Checks if the function returns ordered when an early return occurs.

        let mut single_number_vector = vec![1];
        let mut two_number_vector = vec![2,1];

        let mut single_number_heap_vector = single_number_vector.to_vec();
        let mut two_number_heap_vector = two_number_vector.to_vec();

        double_sort(&mut single_number_vector);
        double_sort(&mut two_number_vector);

        double_heap_sort(&mut single_number_heap_vector);
        double_heap_sort(&mut two_number_heap_vector);

        assert_eq!(single_number_vector,[1]);
        assert_eq!(two_number_vector,[1,2]);


        assert_eq!(single_number_heap_vector,[1]);
        assert_eq!(two_number_heap_vector,[1,2]);
    }

}