// Rust has only one string type in the core language, which is the string slice str that is usually seen in its borrowed form &str.
// string slices, which are references to some UTF-8 encoded string data stored elsewhere. 
//  String literals, for example, are stored in the program’s binary and are therefore string slices.


// The String type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.

pub fn string_type() {
    // This line creates a new empty string called s, which we can then load data into. 
    let mut s = String::new();

    // often we'll have initial data, where we can use to_string
    let s2 = "initial value".to_string();

    // or we can use String from
    let s3 = String::from("initial value");

    // Remember that strings are UTF-8 encoded, so we can include any properly encoded data in them, as shown in Listing 8-14.
    let hello = String::from("Hello");
    println!("{hello}!");
    let hello_arabic = String::from("السلام عليكم");
    println!("hello in Arabic is {hello_arabic}");
    let hello_czech = String::from("Dobrý den");
    println!("hello in Czech is {hello_czech}");
    let hello_hebrew = String::from("שָׁלוֹם");
    println!("hello in Hebrew is {hello_hebrew}");
    let hello_hindu = String::from("नमस्ते");
    println!("hello in Hindu is {hello_hindu}");
    let hello_japanese = String::from("こんにちは");
    println!("hello in Japanese is {hello_japanese}");
    let hello_korean = String::from("안녕하세요");
    println!("hello in Korean is {hello_korean}");
    let hello_chinese = String::from("你好");
    println!("hello in Chinese is {hello_chinese}");
    let hello_portuguese = String::from("Olá");
    println!("hello in Portuguese is {hello_portuguese}");
    let hello_russian = String::from("Здравствуйте");
    println!("hello in Russian is {hello_russian}");
    let hello_spanish = String::from("Hola");
    println!("hello in Spanish is {hello_spanish}");

    // A String can grow in size and its contents can change, just like the contents of a Vec<T>, if you push more data into it. In addition, you can conveniently use the + operator or the format! macro to concatenate String values.
    // We can grow a String by using the push_str method to append a string slice, as shown in Listing 8-15.
    let mut s = String::from("foo");
    // The push_str method takes a string slice because we don’t necessarily want to take ownership of the parameter. 
    s.push_str("bar");
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // &s2 is a &String
    // The reason we’re able to use &s2 in the call to add is that the compiler can coerce the &String argument into a &str. When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..]
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // S2 is still a valid String after operation
    // no & before s1 takes ownership reassigns to s3
    println!("s2 value {s2}");
    println!("s3 value {s3}");

    // multiple concat
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // or simply
    // The format! macro works like println!, but instead of printing the output to the screen, it returns a String with the contents. 
    let s_concat1 = format!("{s1}-{s2}-{s3}");
    println!("{s_concat1}");
    // format! macro uses references so that this call doesn’t take ownership of any of its parameters.
    println!("{s1} s1 I'm still here");
    println!("{s2} s2 I'm still here");
    println!("{s3} s3 I'm still here");


    //  (Note that this string begins with the capital Cyrillic letter Ze, not the number 3.)
    let hello = "Здравствуйте";
    // Asked how long the string is, you might say 12. In fact, Rust’s answer is 24: that’s the number of bytes it takes to encode “Здравствуйте” in UTF-8, because each Unicode scalar value in that string takes 2 bytes of storage. 
    //  Therefore, an index into the string’s bytes will not always correlate to a valid Unicode scalar value. 
    // let answer = &hello[0]; 
    let answer = "151";
    // first byte of 3 is 208
    // letter skips 3 and starts with д, which is 151
    // but when indeixing, we expect 208
    // in fact get 151
    // however neither makes sense to human

    // println!("third letter in {hello} is {answser}");
    // Rust doesn't allow string indexing because
    // The answer, then, is that to avoid returning an unexpected value and causing bugs that might not be discovered immediately, Rust doesn’t compile this code at all and prevents misunderstandings early in the development process.
    // Another point about UTF-8 is that there are actually three relevant ways to look at strings from Rust’s perspective: as bytes, scalar values, and grapheme clusters (the closest thing to what we would call letters).
    // A final reason Rust doesn’t allow us to index into a String to get a character is that indexing operations are expected to always take constant time (O(1)). 
    //     But it isn’t possible to guarantee that performance with a String, because Rust would have to walk through the contents from the beginning to the index to determine how many valid characters there were.

    // Indexing into a string is often a bad idea because it’s not clear what the return type of the string-indexing operation should be: a byte value, a character, a grapheme cluster, or a string slice. 

    // slicing the string
    let hello_str = String::from("дравствуйте");
    let c = &hello_str[0..2]; // first 2 bytes
    println!("the first letter in {s} is {c}");


    let hello_str = String::from("你好");
    let c = &hello_str[0..3]; // first 2 bytes
    println!("the first letter in {s} is {c}");




    let hello_str = String::from("hello");
    let c = &hello_str[0..1]; // first 2 bytes
    println!("the first letter in {s} is {c}");

    // The best way to operate on pieces of strings is to be explicit about whether you want characters or bytes.
    //  For individual Unicode scalar values, use the chars method. Calling chars on “Зд” separates out and returns two values of type char, and you can iterate over the result to access each element:

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }

}
