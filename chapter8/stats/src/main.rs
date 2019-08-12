use std::collections::HashMap;

fn main() {
    let mut v = vec![];
    for i in 1..101 {
        v.push(i);
    };

    // be sure to make them references because
    // we dont want to take ownership
    println!("{}", average(&v));
    println!("{}", median(&v));
    println!("{}", mode(&v));
}

fn average(v: &std::vec::Vec<i32>) -> f32 {
    let mut x = 0.0;
    let length = v.len() as f32;

    for i in v.iter() {
        x += *i as f32;
    }

    x / length
}

fn median(v: &std::vec::Vec<i32>) -> i32 {
    v[v.len()/2]
}

fn mode(v: &std::vec::Vec<i32>) -> i32 {
    let mut map: HashMap<_, _> = HashMap::new();
    
    count_occurences(&mut map, v);
    max(&map)
}

fn count_occurences(map: &mut HashMap<i32, i32>, v: &std::vec::Vec<i32>) -> () {
    for i in v.iter() {
        *(map.entry(*i).or_insert(0)) += 1;
    }
}

fn max(map: &HashMap<i32, i32> ) -> i32 {
    let mut max = (0,0);
    
    for (k, v) in map {
        if *v > max.1 {
            max = (*k,*v);
        };
    }

    max.0
}