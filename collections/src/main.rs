use std::collections::HashMap;

fn average_value(vec: &Vec<i32>) -> f64 {
    let mut result = 0;

    for i in vec {
        result += *i;
    }

    return result as f64 / vec.len() as f64
}

fn median(vec: &Vec<i32>) -> i32 { 
    let mut vec: Vec<i32> = vec.clone();
    vec.sort();

    return vec[vec.len() / 2]
}

fn mode(vec: &Vec<i32>) -> i32 {
    let mut counter = HashMap::new();

    for &i in vec {
        *counter.entry(i).or_insert(0) += 1;
    }

    let mut max_count_value = 0;
    let mut max_count_key = 0;

    for (key, value) in counter {
        if value > max_count_value {
            max_count_value = value;
            max_count_key = key;
        }
    }

    return max_count_key;
}

fn is_latin_word(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_alphabetic())
}

fn str_to_pig_latin(text: &str) -> String {
    if !is_latin_word(&text) {
        return String::from("The word is not English");
    }

    let first_char = match text.chars().next() {
        Some(c) => c,
        None => return String::from("Empty string"),
    };

    let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
    
    if vowels.contains(&first_char) {
        format!("{}-hay", text)
    }
    else {
        let rest: String = text.chars().skip(1).collect();
        format!("{}-{}ay", rest, first_char)
    }
}

fn handle_add(input: &str, company: &mut HashMap<String, Vec<String>>) {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.len() != 4  || parts[0] != "Add" || parts[2] != "to" {
        println!("Формат Add name to Departament");
        return;
    }

    let name = parts[1].to_string();
    let departament = parts [3].to_string();

    company.entry(departament).or_default().push(name);
}

fn handle_list_departament(input: &str, company: &HashMap<String, Vec<String>>) {
    if let Some(employees) = company.get(input) {
        println!("{} : {:?}", input, employees);
    } else {
        println!("Отдел {} пока пуст.", input);
    }
}

fn handle_list_all(company: &HashMap<String, Vec<String>>) {
    let mut departments: Vec<_> = company.keys().cloned().collect();
    departments.sort();

    for dep in departments {
        let mut employees = company[&dep].clone();
        employees.sort();
        println!("{}: {:?}", dep, employees);
    }
}


fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10];
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    println!("Average value: {}", average_value(&v));

    println!("Median: {}", median(&v));
    println!("Mode: {}", mode(&v));

    println!("Pig Latin first: {}", str_to_pig_latin("first"));
    println!("Pig Latin apple: {}", str_to_pig_latin("apple"));
    println!("Pig Latin привет: {}", str_to_pig_latin("привет"));

    
    handle_add("Add Sally to Engineering", &mut company);
    handle_add("Add Amir to Sales", &mut company);
    handle_add("Add Andrew to Sales", &mut company);

    handle_list_departament("Sales", &company);
    handle_list_all(&company);
}
