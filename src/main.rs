

struct Developer{
    lang : String,
    name : String,
    age : i32,
    tasks : Vec<String>
}


impl Developer {
    pub fn view(&self){
        println!("Имя - {}\nЯзык - {}\nВозраст - {}", self.name, self.lang, self.age)
    }

    pub fn push_tasks(&mut self, value: String){
        self.tasks.push(value)
    }
}

fn main(){
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
   Artem.push_tasks("Сделать чай".to_string());
   println!("{}", &Artem.tasks[0]);

   println!("\n\n");

   Yura.view();
   Yura.push_tasks("Пососать хуй".to_string());
   Yura.push_tasks("Пойти нахуй".to_string());
   Yura.push_tasks("Ебнуться головой об стену".to_string());
   for task in &Yura.tasks{
      println!("{}", task) 
   }
   
}