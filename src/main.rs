use std::collections::HashMap;
fn main() {
    //exercise 1 (statistics)
    println!("");
    println!("numbers:");
    let numbers = [1];

    println!("{:?}", numbers);

    println!("median: {}", median(&numbers).map_or_else(|| "no median".to_string(), |num| num.to_string()));
    println!("mode: {}", mode(&numbers).map_or_else(|| "no mode".to_string(), |num| num.to_string()));
    
    //exercise 2 (pig latin)
    println!("");
    println!("pig latin:");
    let words = ["pig", "bell", "man", "george"];
    println!("{:?}", words);

    for word in words {
        println!("{}", pig_latin(word).map_or_else(|e| e.to_string(), |v| v));
    }
}

fn median<T: Clone + Ord>(numbers: &[T]) -> Option<T>{
    if numbers.len() == 0 {
        return None
    }
    let mut vector = numbers.to_vec();
    vector.sort();
    let index = (vector.len() - 1) / 2;
    Some(vector[index].clone())
}

fn mode<T: Eq + std::hash::Hash + Clone>(numbers: &[T]) -> Option<T>{
    let mut occurrences : HashMap<T, u32>= HashMap::new();
    for num in numbers{
        occurrences.entry(num.clone()).and_modify(|m| *m += 1).or_insert(1);
    }
    occurrences.iter().max_by(|(_, v1), (_, v2)| v1.cmp(v2)).map(|(num, _occurrences)| num).cloned()
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
}

fn pig_latin(input: &str) -> Result<String, &str>{
    if !input.chars().all(|c| c.is_alphabetic() || c == ' ') { return Err("not alphabetical") }
    let mut output = input.to_string();
    let first_char = input.chars().next().ok_or("empty string")?;
    if is_vowel(first_char) {
        output += "-hay";
    }
    else {
        output.remove(0);
        output += &format!("-{first_char}ay")
    }
    Ok(output)
}