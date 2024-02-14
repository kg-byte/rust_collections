use std::collections::HashMap;

pub fn exercises() {
    let mut numbers = vec![42, 1, 36, 34, 76, 378, 43, 1, 43, 54, 2, 3, 43];
    let (mid, mode) = find_median_and_mode_of_integer_list(&numbers);
    println!("median of value is {mid}, the mode value is {mode}");
    println!("{:?}", numbers);

    let britta_pig_lattin =  eng_to_pig_latin("not in front of duncan");
    println!("{britta_pig_lattin}");

    command_line_tool();
}


fn find_median_and_mode_of_integer_list(numbers: &Vec<i32>) -> (i32, i32) {
    let median: i32;
    // this will not change the original numbers order
    // the original numbers are borrowed reference non-mutable
    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort();
    let mid_index = numbers.len() / 2;
    let median = sorted_numbers[mid_index];

    let mut mode: i32 = 0;
    let mut max_count = 0;
    let mut count_map: HashMap<i32, usize> = HashMap::new();
    for &num in numbers {
        let count = count_map.entry(num).or_insert(0);
        *count +=1;

        if *count > max_count {
            max_count = *count;
            mode = num;
        }
    }

    (median, mode)
}


fn is_vowel(c: char) -> bool {
    match c.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}


fn pig_latinize(word: &str) -> String {
    let mut chars = word.chars();

    if let Some(first_char) = chars.next() {
        if is_vowel(first_char) {
            // word starts with a vowel, add 'hay' to the end
            format!("{}-hay", word)
        }
        else {
            // word  starts with a consonant, move it to the end and add 'ay'.
            let rest_of_word: String = chars.collect();
            format!("{}-{}ay", rest_of_word, first_char)
        }
    } else {
        // empty string, return as is
        word.to_string()
    }
}

fn eng_to_pig_latin(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| pig_latinize(word))
        .collect::<Vec<String>>()
        .join(" ")
}


fn command_line_tool() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Enter a command (e.g. 'Add Sally to Enginering' or 'List Engineering'):");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.len() < 2 {
            println!("Invalid command. Please use 'Add <Name> to <Department>' or 'List <Department>'");
            continue;
        }

        match parts[0] {
            "Add" => {
                let name = parts[1].to_string();
                let department = parts.last().unwrap().to_string();
                company
                    .entry(department.clone())
                    .or_insert(Vec::new())
                    .push(name.clone());
                println!("Added {} to {}", name, department);
            }
            "List" => {
                let department = parts.last().unwrap();
                if department == &"company" {
                    list_company(&company);
                } else {
                    list_department(&company, department);
                }
            }
            _ => println!("Invalid command. Please use 'Add <Name> to <Department>' or 'List <Department>'"),
        }
   
    }
}

fn list_department(company: &HashMap<String, Vec<String>>, department: &str) {
    if let Some(employees) = company.get(department) {
        let mut sorted_employees = employees.clone();
        sorted_employees.sort();
        println!("Employees in {} (sorted alphabetically): {:?}", department, sorted_employees);
    } else {
        println!("Department {} does not exist.", department);
    }
}

fn list_company(company: &HashMap<String, Vec<String>>) {
    let mut all_employees: Vec<&str> = Vec::new();
    
    for employees in company.values() {
        all_employees.extend_from_slice(&employees.iter().map(|s| s.as_str()).collect::<Vec<&str>>());
    }

    all_employees.sort();
    println!("Employees in the company (sorted alphabetically): {:?}", all_employees);
}