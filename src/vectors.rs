pub fn vector() {
    // The first collection type we’ll look at is Vec<T>, also known as a vector. 
    // Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory. 
    // Vectors can only store values of the same type. They are useful when you have a list of items, such as the lines of text in a file or the prices of items in a shopping cart.
   
    // Note that we added a type annotation here. Because we aren’t inserting any values into this vector, Rust doesn’t know what kind of elements we intend to store
    let mut v1: Vec<i32> = Vec::new();
    v1.push(5);
    v1.push(6);
    v1.push(7);
    let third = v1.get(2);
    // get method returns None without panicking
    // use if this may happen occasionally under normal circumstances
    match third {
        Some(third) => println!("The third element of v1 is {third}"),
        None => println!("There is no third element")
    }

    // More often, you’ll create a Vec<T> with initial values and Rust will infer the type of value you want to store, so you rarely need to do this type annotation.
    // Infers default integer time i32
    let v2 = vec![1, 2, 3, 4, 5];
    // &v2[index] will panic if reference is out of bound
    // use when you want program to crash
    println!("The third element of v2 is {}", &v2[2]);



    // THIS CRASHES on 'cannot borrow `v` as mutable because it is also borrowed as immutable' 
    let mut v4 = vec![1, 2, 3, 4, 5];

    //let first = &v[0];

    //v.push(6);

    // println!("The first element is: {first}");

    // This error is due to the way vectors work: 
    //   because vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space,
    //   if there isn’t enough room to put all the elements next to each other where the vector is currently stored. In that case, the reference to the first element would be pointing to deallocated memory. 
    //   The borrowing rules prevent programs from ending up in that situation.

    // iterating over values in a vector
    let v5 = vec![100, 32, 57];
    for i in &v5 {
        println!("{i}")
    }

    // iterate over mutable refrences
    let mut v6 = vec![100, 32, 57];
    for i in &mut v6 {
        // Iterating over a vector, whether immutably or mutably, is safe because of the borrow checker's rules. 
        println!("i before mutation +50: {i}");
        // To change the value that the mutable reference refers to, we have to use the * dereference operator to get to the value in i
        *i +=50;
        println!("i after mutation +50: {i}");
    }
}