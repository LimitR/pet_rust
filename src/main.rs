use std::io::{self, Write};
use std::fs::File;


fn main(){
    let mut x: Vec<String> = Vec::new();
    let mut final_vector: Vec<&str> = Vec::new();
    let mut out_value = String::new();
    let stdin = io::stdin();
    loop{
        stdin.read_line(&mut out_value).expect("ERROR");
    //    io::stdin().read_line(&out_value)?; 
       if out_value.trim() == "ok".to_string(){
           let mut file = File::create("foo.txt").expect("ERROR CREATE FILE");
           for value in &x{
            final_vector.push(&value[0..value.len()-2]);
            file.write(value.as_bytes()).unwrap();
           };
           println!("{:?}", &final_vector);
           
           break;
       };
       x.push(out_value);
       out_value = "".to_string();
    }
    
}


