

pub fn demo_it() {
    demo_hashmap_simple();    
    demo_get_from_hashmap();
    demo_update_value_of_hashmap();
    demo_word_count();
}

fn demo_hashmap_simple() {
    use std::collections::HashMap;

    //create hashmap via insert
    let team_list = vec![
        ("China".to_string(), 100),
        ("USA".to_string(), 10),
        ("Japan".to_string(), 50),
    ];

    let mut teams_map = HashMap::new();
    for team in &team_list{
        teams_map.insert(&team.0, team.1);
    }
    println!("{:?}", teams_map);

    let teams_map2: HashMap<_, _> = team_list.into_iter().collect();

    println!("{:?}", teams_map2);
}

fn demo_get_from_hashmap() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert("blue".to_string(), 123);
    scores.insert("red".to_string(), 456);

    let color = String::from("red");
    let found = scores.get(&color);
    let not_found = scores.get("not here");
    println!("found = {:?}", found);
    println!("not found = {:?}", not_found);

    //traverse
    for (key, value) in scores {
        println!("key={}, value={}", key, value);
    }
}

fn demo_update_value_of_hashmap() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert("Blue", 20);
    
    //overwrite existing value
    let old = scores.insert("Blue", 80);
    println!("insert again returns old value: {:?}", old);
    if let Some(new) = scores.get("Blue") {
        println!("new value is {}", new);
    } else {
        println!("new value is None");
    }

    //get or insert
    let v = scores.entry("Yellow").or_insert(12);
    println!("yellow={:?}", *v);

    let v = scores.entry("Yellow").or_insert(100);
    println!("yellow={:?}", *v);
}

//update by existing value
fn demo_word_count() {
    use std::collections::HashMap;

    let text = "Hello, world, this is wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count +=1;
    }
    println!("{:?}", map);
}