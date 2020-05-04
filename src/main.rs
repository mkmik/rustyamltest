use std::env;
use std::fs;
use yaml_rust::{YamlEmitter, YamlLoader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!(
        "read {} into string; ascii: {}",
        filename,
        contents.is_ascii(),
    );

    let docs = YamlLoader::load_from_str(&contents).unwrap();
    for doc in docs.iter() {
        println!("Doc: {:?}", doc);

        let mut out_str = String::new();
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap(); // dump the YAML object to a String

        println!("{}", out_str);
    }
}
