use std::fs;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;






fn main() {

    // let testing_files_path = Path::new("/Users/apples/Desktop/TestingFiles/poems.txt");
    // let desktopPath = Path::new("/Users/apples/Desktop");

    
    // for entry in fs::read_dir(desktopPath).expect("dd"){
    //     let dir = entry.expect("cc");
    //     println!("{:?}", dir.path());
    // }
    // // // let metadata = testing_files_path.metadata().expect("xx");
    // // println!("{}", metadata.is_dir());
    
    // let mut file = fs::File::open("/Users/apples/Desktop/Rust/Playground/16Nov2018/having_fun/info.txt").expect("Error opening file");
    // let mut contents = String::new();
    // let mut create_file = fs::File::create(testing_files_path).expect("Create file error");
    // file.read_to_string(&mut contents).expect("Oop! Cannot read the fifle");
    // create_file.write(contents.as_bytes());
    //tutorial_code();
    // tutorial_code_hash_maps()
    //tutorial_code_optional_binding()
    tutorial_code_casting()
    
}
#[derive(Debug)]
enum Example {
    Float(f64),
    Int(i32),
    Text(String),
}

fn tutorial_code() {
    let mut family = Vec::new();
    family.push("Chris");
    family.push("Carlton");
    family.push("Kiannah");
    family.push("Mom");
    family.push("Ryan");
    for member in &family {
        println!("Family Member {}", member);
    }

    let mut examples = Vec::new();
    let one_example = Example::Int(143);
    let two_example = Example::Float(12.32);
    let three_example = Example::Text(String::from("Chris"));

    examples.push(one_example);
    examples.push(two_example);
    examples.push(three_example);
    println!("{:?} {} {}", &examples, examples.len(), examples.capacity())
}

fn tutorial_code_hash_maps() {
    let mut hm = HashMap::new();
    hm.insert(String::from("random"), 12);
    hm.insert(String::from("strings"), 49);

    for (k,v) in &hm {
        println!("{} {}", k, v);
    }

    match hm.get(&String::from("random")) {
        Some(&n) => println!("{}", n),
        _ => println!("No match"),
    }

    hm.remove(&String::from("random"));
}


fn tutorial_code_optional_binding() {
    let mut s = Some(10);

    while let Some(i) = s {
        if i > 100 {
            println!("Quit");
            s = None;
        } else {
            println!("{}", i);
            s = Some(i + 10);
        }
    }
}

fn tutorial_code_casting() {
    let f = 24.212312313212;
    let i = f as u8;
    let c = i as char;

    println!("{} {} {}", f, i, 14 as char);
}