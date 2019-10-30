
use std::env;

// looks for the last file
mod crypto;
mod reqwest;
mod login;

struct ToDoItem{
    name: String,
    completed: char
}

struct ToDoList{
    list:Vec<ToDoItem>
}


impl ToDoItem {
    fn new(name:String)-> ToDoItem{
        return ToDoItem{
            name: name,
            completed:' '
        };
    }
}

impl ToDoList {
    fn new()-> ToDoList{
        return ToDoList{
            list: Vec::new()
        }
    }

    fn add_to_list(&mut self, name: String ){
        let todo_item=ToDoItem::new(name);
        self.list.push(todo_item);

    }

    fn print(&self){
        for(index,item)  in self.list.iter().enumerate() {
          println!("[{}] - {} - {} -{:?}",index, item.name,item.completed,crypto::hash("test".to_string()))
        }
    }

    fn remove(&mut self, index:usize){
        self.list.remove(index);
    }
}

enum Command{
    Get,
    Add(String),
    Done(usize)
}

fn main() {

    // println!("{:#?}",crypto::hash("test".to_string()));
    // reqwest::test();
login::login_inputs();
let args: Vec<String> = env::args().collect();
// println!("{:#?}", args);
let command = match args[1].as_str() {
    "get"=>Command::Get,
    "add"=> Command::Add(args[2].clone()),
    "done"=>Command::Done(args[2].parse().expect("something")),
    _=> panic!("Please provide a valid command")
};

let mut todo_list=ToDoList::new();

// let todo_list= vec![todo_item_1,todo_item_2];
todo_list.add_to_list("Make a coffee".to_string());
todo_list.add_to_list("Learn Rust lang".to_string());


match command {
    Command::Get => todo_list.print(),
    Command::Add(_task)=>{
        let task=args[2].clone();
        todo_list.add_to_list(task);
        todo_list.print()
    },
    Command::Done(index) => {
        todo_list.remove(index);
        todo_list.print()
    }
}
}
