use std::collections::HashMap;

fn stats() {
    let mut v: Vec<i32> = vec![1, 5, 4, 2, 7, 2];
    let count = v.len();

    ////////////
    let average = v.iter()
        .fold(0, |acc, x| acc + x) as f32
        / (count as f32);
    
    println!("average: {}", average);
    ////////////
    
    ////////////
    v.sort();
    // 6
    
    let median = if count % 2 == 0 {
            let lower_index = (count / 2) - 1;
            let median_a = v[lower_index];
            let median_b = v[lower_index + 1];
            ((median_a + median_b) as f32) / 2.0
    } else {
        v[count / 2 as usize] as f32
    };
    
    println!("median: {}", median);
    ////////////
    
    ////////////
    let mut buckets = HashMap::new();
    for i in v.iter() {
        let prev_val = buckets.get(i).unwrap_or(&0) + 1;
        buckets.insert(i, prev_val);
    }
    
    let (mut max_val, mut max_count) = (0, 0);
    for (val, count) in buckets.iter() {
        if *count > max_count {
            max_val = **val;
            max_count = *count;
        }
    }
    
    println!("buckets: {:?}", buckets);
    println!("mode: {}", max_val);
    ////////////
}

enum FirstChar {
    Unknown,
    Vowel,
    Consonant(char),
}

fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

fn pig_latin(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 5);
    
    let mut first = FirstChar::Unknown;
    for c in s.chars() {
        match first {
            FirstChar::Unknown if is_vowel(c) => {
                first = FirstChar::Vowel;
                out.push(c);
            },
            FirstChar::Unknown => {
                first = FirstChar::Consonant(c);
            },
            _ => {
                out.push(c);
            }
        }
    }
    
    match first {
        FirstChar::Consonant(c) => {
            out += "-";
            out.push(c);
            out += "ay";
        },
        FirstChar::Vowel => {
            out += "-hay";
        },
        _ => {}
    }
    out
}

fn main() {
    // stats();
    let output = pig_latin("hello");
    println!("{}", output);
}
