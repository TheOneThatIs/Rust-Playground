fn main() {
    println!("Hello, world!");
}

fn example_1(array_of_ints: &[i32], n: usize) -> i32 {
    let mut currmin = 100;
    for i in 0..n {
        if array_of_ints[i] < currmin {
            currmin = array_of_ints[i];
        }
    }
    currmin
}

fn example_2(array_of_ints: &Vec<i32>) {
    print!("[");
    for i in 0..array_of_ints.len() {
        print!("{}", array_of_ints[i]);
        if i < array_of_ints.len() - 1 {
            print!(", ");
        }
    }
    print!("]");
}

fn example_3(array_of_ints: &[i32]) -> bool{
    let a: i32 = 10;
    let b: i32 = 5;
    let mut found: bool = false;

    for i in 0..array_of_ints.len() {
        if a == array_of_ints[i] {
            println!("The value of 'a' was found in the int array.");
            found = true;
        }
        else if b == array_of_ints[i] {
            println!("The value of 'b' was found in the int array.");
            found = true;
        }
    }
    if !found {
        println!("None of the search values were found in the array.");
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_smallest() {
        let arr: Vec<i32> = vec![12, 3, 5, 7, 190, 2, 10]; // Create a vector
        let result = example_1(&arr, 4); // Grab the result from the function when n is 4
        assert_eq!(result, 3); // If result is the 3, then the assertion passes, as the two is past index 4-1
    }

    #[test]
    fn test_search_smallest_with_negative() {
        let arr: Vec<i32> = vec![12, 3, -5, 7, 190, -10]; // Create the vector
        let result = example_1(&arr, 5); // Grab the result from the function when n is 5
        assert_eq!(result, -5); // If result is the -5, then the assertion passes, as the -10 is past index 5-1

        let result = example_1(&arr, 6);
        assert_eq!(result, -10); // Checks that it still works if set to check up to index 6. I should be -10 instead of -5 now that the former is included.
    }
    #[test]
    fn test_search_empty_array() {
        let arr: Vec<i32> = vec![]; // Create the empty vec
        let result = example_1(&arr, 0);
        assert_eq!(result, 100) // It passes if it gives the default value
    }
    #[test]
    fn test_printing() {
        let arr: Vec<i32> = vec![12, 3, 5, 7];
        println!("The expected result:\n{:?}", arr); // Print the vector with the debug format specifier
        println!("The actual result:");
        example_2(&arr); // Print the result of the function. You have to manually check through the console to ensure that they are the same.
    }
    #[test]
    fn test_printing_empty_array() {
        let arr: Vec<i32> = vec![];
        println!("The expected result:\n{:?}", arr); // Print the vector with the debug format specifier
        println!("The actual result:");
        example_2(&arr); // Print the result of the function. You have to manually check through the console to ensure that they are the same.
    }

    #[test]
    fn test_contains_5_or_10() {
        let mut arr: Vec<i32> = vec![12, -3, 5, 72, 10, 15];
        assert_eq!(example_3(&arr), true);
        arr.retain(|&x| x != 10); // Remove 10 from the vec
        assert_eq!(example_3(&arr), true); // Should be true as the 5 still exists
        arr.retain(|&x| x != 5); // Remove 5 from the vector
        assert_eq!(example_3(&arr), false); // Should evaluate to false as 5 and 10 are no longer in the vec
    }
    #[test]
    fn test_contains_5_or_10_with_empty_array() {
        let arr: Vec<i32> = vec![]; // Empty vector
        assert_eq!(example_3(&arr), false); // An empty Vector clearly does not have either a 5 or a 10.
    }
}
