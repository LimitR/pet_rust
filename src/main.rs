pub(crate) use std::{fs::File, io::stdin, io::stdout, io::Write};
use crossterm::QueueableCommand;
use crossterm::event::{MouseEvent, MouseEventKind, poll};
use crossterm::{
    cursor::{DisableBlinking, EnableBlinking, MoveTo, RestorePosition, SavePosition, position},
    event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};
use serde::Deserialize;
use std::time::Duration;
//cd Desktop\rust_pet_project\pet_project
fn main()  -> Result<()> {
    let mut x: Vec<String> = Vec::new();
    let mut final_vector: Vec<&str> = Vec::new();
    let mut out_value = String::new();
    let stdin = stdin();
    let mut stdout = stdout();
    execute!(stdout, EnableMouseCapture)?;
    loop{
        let event = read()?;
        // let testing = Event::Key(KeyCode::Char('c'));
        
        if let Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            ..
        }) = event{
            println!("_______");
        }

        if let Event::Mouse(MouseEvent {
            column: 64,
            row: 13,
            ..
        }) = event{
            println!("____MOUSES___");
        }

        println!("{:?}", event);
        
        // if event == Event::Mouse(MouseEvent){
        //     println!("TESTESTESTSET");
        // }
        
        if event == Event::Key(KeyCode::Esc.into()) {
            
            break;
        }
    }
    // loop {
    //     stdin.read_line(&mut out_value).expect("ERROR");
    //     if out_value.trim() == "ok".to_string() {
    //         let mut file = File::create("foo.txt").expect("ERROR CREATE FILE");
    //         for value in &x {
    //             final_vector.push(&value[0..value.len() - 2]);
    //             file.write(value.as_bytes()).unwrap();
    //         }
    //         stdout()
    //                 .execute(SetForegroundColor(Color::Red))?
    //                 .execute(SetBackgroundColor(Color::Green))?
    //                 .execute(Print(&final_vector.join(" | ")))?
    //                 .execute(ResetColor)?;
    //         break;
    //     };
    //     x.push(out_value);
    //     out_value = "".to_string();
    // }
    // out_value = "".to_string();
    // loop{
    //     stdin.read_line(&mut out_value).expect("ERROR");
    //     if out_value.trim() == "end".to_string() {
    //         break;
    //     }
    // }
    Ok(())
}
