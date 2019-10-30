extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use console::style;

pub fn login_inputs(){
    // `()` can be used when no completer is required
let mut rl = Editor::<()>::new();
if rl.load_history("history.txt").is_err() {
    println!("No previous history found.");
}

println!("Enter username");
let username=rl.readline(">> ");
println!("Enter Password");
let password=rl.readline(">> ");
println!("{:?} {:?}",style(username).cyan(),password);

    // match username {
    //     Ok(line) => {
    //         rl.add_history_entry(line.as_str());
    //         println!("Line: {}", line);
    //     },
    //     Err(ReadlineError::Interrupted) => {
    //         println!("CTRL-C");
    //
    //     },
    //     Err(ReadlineError::Eof) => {
    //         println!("CTRL-D");
    //
    //     },
    //     Err(err) => {
    //         println!("Error: {:?}", err);
    //
    //     }
    // }

rl.save_history("history.txt").unwrap();
}
