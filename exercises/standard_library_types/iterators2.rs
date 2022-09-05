// iterators2.rs
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a hint.

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();

    if c.as_str().is_empty() {
        return String::new();
    }
    let mut temp = c.next().unwrap().to_uppercase().to_string();
    for iter_ch in c {
        //println!("{:?} {:?}", iter_ch.is_ascii(), iter_ch);
        temp.push(iter_ch);
    }
   
    temp.to_string()
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut v =  Vec::new();

    for iter in words {
       // let result = capitalize_first(iter).to_string();
        //println!("{result}");
        v.push(capitalize_first(iter).to_string());
        
    }
    v
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    let mut v = String::new();
    for iter in words {        
        //println!("{result}");        
    v.push_str(capitalize_first(iter).as_str());
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
