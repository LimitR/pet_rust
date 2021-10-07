use std::io;

struct Developer{
    lang : String,
    name : String,
    age : i32,
    tasks : Vec<String>
}


impl Developer {
    pub fn view(&self){
        println!("##################\nИмя - {}\nЯзык - {}\nВозраст - {}\n##################", self.name, self.lang, self.age)
    }

    pub fn push_tasks(&mut self, value: String){
        self.tasks.push(value)
    }
}

fn main(){
    
    let mut vec_tasks = Vec::new();
    let mut args = String::new();

    loop{
    io::stdin()
        .read_line(&mut args)
        .expect("Failed to read line");
        if args.trim() == "ok"{
            args = "".to_string();
            break;
        }else{
            vec_tasks.push(args.clone());
            args = "".to_string();
            println!("Введите `OK` если все готово")
        }
    };

    

    let mut index_tasks: i32 = 0;
   let mut Artem = Developer{
       lang : "Node.js".to_string(),
       name : "Artem".to_string(),
       age : 22,
       tasks : Vec::new()
   };
   let mut Yura = Developer{
        lang : "3D MAX".to_string(),
        name : "Юра".to_string(),
        age : 21,
        tasks : Vec::new()
   };
   Artem.view();
   println!("\nЗадачи:");
   for task in &vec_tasks{
    index_tasks = &index_tasks + 1;
   println!("{}. {}",&index_tasks, task) 
}
index_tasks = 0;

   loop{
    io::stdin()
        .read_line(&mut args)
        .expect("Failed to read line");
        if args.trim() == "ok"{
            args = "".to_string();
            break;
        }else{
            vec_tasks.push(args.clone());
            args = "".to_string();
            println!("Введите `OK` если все готово")
        }
    };
    println!("\n\n");

    Yura.view();
   println!("\nЗадачи:");
   for task in &vec_tasks{
       index_tasks = &index_tasks + 1;
      println!("{}. {}",&index_tasks, task) 
   }
   
}