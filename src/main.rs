use std::collections::HashMap;

fn main() {

    // like a Dictionary<T,T>
    let mut hm: HashMap<i32, String> = HashMap::new();
    hm.insert(1, String::from("ABC"));
    hm.insert(2, String::from("DEF"));
    hm.insert(3, String::from("GHI"));
    let key = 4;
    let val = String::from("JKL");
    hm.insert(key, val.clone());
    let _v = val;
    match hm.get(&22) {
        Some(val) => println!("Key 2 is {}", val),
        None => println!("Key 2 Not found"),
    };
    hm.entry(88).or_insert(String::from("XYZ"));
    for (key, val) in hm {
        println!("{}:{}, ", key, val);
    }

    // like a List<T>
    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);
    println!("{:?}", vec);

    let _v = vec![0, 1, 1, 2, 3, 5, 8, 13];
}
