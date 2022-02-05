pub(crate) use std::{fs::File, io::stdin};
use std::io;
use std::io::{stdout, Write};
use crossterm::event::{MouseEvent, MouseEventKind, MouseButton};
use crossterm::style::Colorize;
use crossterm::{
    queue,
    execute,
    style,
    event::{read, EnableMouseCapture, Event, KeyCode, KeyEvent},
    Result,
    cursor
};



fn main()  -> Result<()> {
    execute!(stdout(), EnableMouseCapture)?;

    let mut stdout = stdout();
    let kist = "●".white();
    let mut color_cursor = style::PrintStyledContent( kist);

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
        if let Event::Key(KeyEvent { code: KeyCode::Enter, ..}) = event_win{
            let mut result = File::create("./pic.txt").unwrap();
            let mut text = String::from("");
            io::stdin().read_line(&mut text).unwrap();
            result.write_all(text.clone().as_bytes()).unwrap();
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

        if let Event::Mouse(MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 0,
            ..
        }) = event_win {
            color_cursor = style::PrintStyledContent( kist.red());
        }

        if event_win == Event::Key(KeyCode::Esc.into()) {
            break;
        }
        stdout.flush().unwrap();
    }

    Ok(())
}