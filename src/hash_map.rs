use std::collections::HashMap;

pub fn hash_map() {
    let mut scores = HashMap::new();
    // Just like vectors, hash maps store their data on the heap.
    // This HashMap has keys of type String and values of type i32. 
    // Like vectors, hash maps are homogeneous: all of the keys must have the same type as each other, and all of the values must have the same type.

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = "Blue".to_string();
    let score = scores.get(&team_name).copied().unwrap_or(0);
    //  The get method returns an Option<&V>
    // if there’s no value for that key in the hash map, get will return None. 
    // This program handles the Option by calling copied to get an Option<i32> rather than an Option<&i32>
    // hen unwrap_or to set score to zero if scores doesn't have an entry for the key.
    println!("Score for {team_name} is {score}");

    for (key, value) in &scores {
        println!("{key} team's score is {value}")
    }

    // For types that implement the Copy trait, like i32, the values are copied into the hash map.
    // For owned values like String, the values will be moved and the hash map will be the owner of those values, as demonstrated in Listing 8-22.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // println!("{field_name}");
    // error: value borrowed after move

    // overwriting a value
    scores.insert(String::from("Blue"), 25);
    // the original value of 10 has been overwritten
    println!{"{:?}", scores};

    scores.entry(String::from("Red")).or_insert(100);
    scores.entry(String::from("Yellow")).or_insert(100);
    // or_insert only inserts when no value is present, yellow 50 is not overwritten
    // red key didn't exist, or_insert inserts 100
    println!{"{:?}", scores};

    updating_a_value_based_on_the_old_value();
}

fn updating_a_value_based_on_the_old_value() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // The or_insert method returns a mutable reference (&mut V) to the value for the specified key.
        let count = map.entry(word).or_insert(0);
        // Here we store that mutable reference in the count variable, so in order to assign to that value, we must first dereference count using the asterisk (*).
        // The mutable reference goes out of scope at the end of the for loop, so all of these changes are safe and allowed by the borrowing rules.
        *count +=1;
    }
    println!("{:?}", map);
}

// By default, HashMap uses a hashing function called SipHash that can provide resistance to Denial of Service (DoS) attacks involving hash tables1.
// This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it.
// If you profile your code and find that the default hash function is too slow for your purposes, you can switch to another function by specifying a different hasher.
// A hasher is a type that implements the BuildHasher trait.
