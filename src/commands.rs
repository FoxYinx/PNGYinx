use std::fs;
use std::str::FromStr;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

pub fn encode(path: &String, key: &String, message: &String) {
    let data = fs::read(path).expect("Unable to read file");
    let png = Png::try_from(data.as_slice());
    if png.is_err() {
        panic!("Le png donné est corrompu")
    }
    let mut png = png.unwrap();
    let chunk_type = ChunkType::from_str(key);
    if chunk_type.is_err() {
        panic!("La clé donnée est incorrecte")
    }
    let chunk_type = chunk_type.unwrap();
    let chunk = Chunk::new(chunk_type, message.clone().into_bytes());
    png.append_chunk(chunk);
    fs::write(path, png.as_bytes()).expect("Unable to write file")
}

pub fn decode(path: &String, key: &String) {
    let data = fs::read(path).expect("Unable to read file");
    let png = Png::try_from(data.as_slice());
    if png.is_err() {
        panic!("Le png donné est corrompu")
    }
    let png = png.unwrap();
    let result = png.chunk_by_type(key);
    match result {
        Some(e) => println!("Message: {}", e.data_as_string().unwrap()),
        None => println!("Aucun message ne fut trouvé")
    }
}

pub fn remove(path: &String, key: &String) {
    let data = fs::read(path).expect("Unable to read file");
    let png = Png::try_from(data.as_slice());
    if png.is_err() {
        panic!("Le png donné est corrompu")
    }
    let mut png = png.unwrap();
    let result = png.remove_chunk(key);
    match result {
        Ok(_chunk) => {
            fs::write(path, png.as_bytes()).expect("Unable to write file");
            println!("The secret message has been successfully deleted!")
        },
        Err(e) => println!("{}", e)
    }
}