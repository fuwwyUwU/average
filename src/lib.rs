use std::collections::HashMap;

pub fn get_numbers(numbers_string: String) -> Result<Vec<i32>, String> {
    let mut numbers: Vec<i32> = Vec::new();

    for i in numbers_string.split_whitespace() {
        numbers.push(match i.parse::<i32>() {
            Ok(val) => val,
            Err(_) => return Err("failed to parse".to_string()),
        })
    }
    Ok(numbers)
}

pub fn mean(numbers: &Vec<i32>) -> i32 {
    let mut total: i32 = 0;

    for i in numbers {
        total += i;
    }

    total / numbers.len() as i32
}

pub fn median(numbers: &mut Vec<i32>) -> f32 {
    numbers.sort();

    // * `vec.len() / 2` is reusable code; so make it it's own varible
    // * Instead of using `vec.get(index).unwrap()`, use `vec[index]`,
    //   as we know our index is valid.
    let index = numbers.len() / 2;

    if numbers.len() % 2 == 1 {
        numbers[index] as f32
    } else {
        (numbers[index - 1] as f32 + numbers[index] as f32) / 2.0
    }
}

pub fn mode(numbers: &Vec<i32>) -> &i32 {
    let mut table = HashMap::new();

    for i in numbers {
        let count = table.entry(i).or_insert(0);
        *count += 1;
    }

    let mut most_common= None;
    
    for (key, value) in table {
        match most_common {
            Some(v) => {
                if v < &value {
                    most_common = Some(key);
                }
            },
            None => {
                most_common = Some(key);
            }
        }
    }

    if let Some(v) = most_common {
        return v
    } else {
        panic!("something went wrong :/")
    }
}

