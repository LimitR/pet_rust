pub(crate) use std::{fs::File, io::stdin};
use std::io::{stdout, Write};
use crossterm::QueueableCommand;
use crossterm::event::{MouseEvent, MouseEventKind, poll, MouseButton};
use crossterm::style::Colorize;
use crossterm::{
    terminal,
    queue,
    execute,
    style,
    cursor::{DisableBlinking, EnableBlinking, MoveTo, RestorePosition, SavePosition, position},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
    cursor
};
use std::time::Duration;
//cd Desktop\rust_pet_project\pet_project


const MENU: &str = "red
blue
black";


fn main()  -> Result<()> {
    let stdin = stdin();
    execute!(stdout(), EnableMouseCapture)?;
    for x in 0..150{
        println!(" ");
    }
    let mut stdout = stdout();
    let mut kist = "●".white();
    let mut color_cursor = style::PrintStyledContent( kist);   
    let mut position_cursor: (u16, u16) = (0, 0);
    let mut position_cursor_y: u16 = 0;

    // for line in MENU.split('\n') {
    //     queue!(stdout, style::Print(line), cursor::MoveToNextLine(1))?;
    // };

    /*
    printin!(stdout, cursor::MoveTo(1, 30), style::PrintStyledContent( "Red".red()))?;
    queue!(stdout, cursor::MoveTo(10,30), style::PrintStyledContent( "White".white()))?;
    queue!(stdout, cursor::MoveTo(20,30), style::PrintStyledContent( "Blue".blue()))?;
    queue!(stdout, cursor::MoveTo(30,30), style::PrintStyledContent( "Black".black()))?;
     */
    loop{     
        
        let event_win = read()?;

        if let Event::Key(KeyEvent { code: KeyCode::Char('0'), .. }) = event_win{
            color_cursor = style::PrintStyledContent( kist.white());
        }
        if let Event::Key(KeyEvent { code: KeyCode::Char('1'), .. }) = event_win{
            color_cursor = style::PrintStyledContent( kist.red());
        }
        if let Event::Key(KeyEvent { code: KeyCode::Char('2'), .. }) = event_win{
            color_cursor = style::PrintStyledContent( kist.blue());
        }
        if let Event::Key(KeyEvent { code: KeyCode::Char('3'), .. }) = event_win{
            color_cursor = style::PrintStyledContent( kist.magenta());
        }
        if let Event::Key(KeyEvent { code: KeyCode::Char('4'), .. }) = event_win{
            color_cursor = style::PrintStyledContent( kist.black());
        }

        if let Event::Mouse(MouseEvent {
        kind: MouseEventKind::Drag(MouseButton::Left),
            column: x,
            row: y,
            ..
        }) = event_win {

                queue!(stdout, cursor::MoveTo(x,y), color_cursor)?;
        }

        if let Event::Mouse(MouseEvent {
            kind: MouseEventKind::Drag(MouseButton::Right),
                column: x,
                row: y,
                ..
            }) = event_win {
    
                    queue!(stdout, cursor::MoveTo(x,y), style::PrintStyledContent( kist.black()))?;
            }


        if let Event::Mouse(MouseEvent { 
            kind: MouseEventKind::Down(MouseButton::Left), 
            column: x, 
            row: y,  
            .. 
        }) = event_win {

            queue!(stdout, cursor::MoveTo(x,y), color_cursor)?;
        }
        if let Event::Mouse(MouseEvent { 
            kind: MouseEventKind::Up(MouseButton::Left), 
            column: x, 
            row: y,
            .. 
        }) = event_win {
            queue!(stdout, cursor::MoveTo(x, y), color_cursor)?;
        }



            if let Event::Resize(x, y) = event_win{
                // println!("x - {}\ny - {}\n", x , y);
                position_cursor.0 = x;
                position_cursor.1 = y;
                position_cursor_y = y;
                queue!(stdout, cursor::MoveTo(0,y), style::PrintStyledContent( "TEST".white()))?;
                queue!(stdout, cursor::MoveTo(10,y), style::PrintStyledContent( "TEST_2".white()))?;
                queue!(stdout, cursor::MoveTo(20,y), style::PrintStyledContent( "TEST_3".white()))?;
            }

            if let Event::Mouse(MouseEvent { 
                kind: MouseEventKind::Down(MouseButton::Left), 
                column: 0, 
                row: position_cursor_y,  
                .. 
            }) = event_win {
    
                color_cursor = style::PrintStyledContent( kist.red());
            }
        
   
        if event_win == Event::Key(KeyCode::Esc.into()) {
            break;
        }
    }

    Ok(())
}

/*
            // if x > position_cursor.0 && y ==  position_cursor.1{
            //     kist = "―"
            // }else if x < position_cursor.0 && y ==  position_cursor.1{
            //     kist = "―"
            // }

            // if x == position_cursor.0 && y <  position_cursor.1{
            //     kist = "│"
            // }else if x == position_cursor.0 && y >  position_cursor.1{
            //     kist = "│"
            // }




            // if x < position_cursor.0 && y <  position_cursor.1{
            //     kist = "╱"
            // }else if x > position_cursor.0 && y >  position_cursor.1{
            //     kist = "╱"
            // }

            // if x > position_cursor.0 && y >  position_cursor.1{
            //     kist = "╲"
            // }else if x < position_cursor.0 && y <  position_cursor.1{
            //     kist = "╲"
            // }

*/