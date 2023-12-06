use clap::Parser;
use std::io::Write;

const TITLE: &str = "Rano";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(default_value = "New Buffer")]
    file: String,
}

fn setup() {
    crossterm::terminal::enable_raw_mode().ok();
    crossterm::execute!(
        std::io::stdout(),
        crossterm::terminal::SetTitle(TITLE),
        crossterm::terminal::EnterAlternateScreen,
        crossterm::style::SetBackgroundColor(crossterm::style::Color::Black),
        crossterm::style::SetForegroundColor(crossterm::style::Color::White)
    )
    .unwrap();
}

fn exit() {
    crossterm::terminal::disable_raw_mode().ok();
    crossterm::execute!(
        std::io::stdout(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::cursor::Show,
        crossterm::style::SetBackgroundColor(crossterm::style::Color::Black),
        crossterm::style::SetForegroundColor(crossterm::style::Color::White)
    )
    .unwrap();
}

fn draw_title(file: &str, modified: bool) {
    let modified_str = if modified { " * " } else { "" };

    let cursor = crossterm::cursor::position().unwrap();
    crossterm::execute!(
        std::io::stdout(),
        crossterm::cursor::MoveTo(0, 0),
        crossterm::style::SetBackgroundColor(crossterm::style::Color::Blue),
        crossterm::style::SetForegroundColor(crossterm::style::Color::White),
        crossterm::cursor::Hide
    )
    .unwrap();

    print!(
        "{}",
        std::iter::repeat(" ")
            .take((crossterm::terminal::size().unwrap().0).into())
            .collect::<String>()
    );

    crossterm::execute!(std::io::stdout(), crossterm::cursor::MoveTo(0, 0),).unwrap();

    print!("{}", TITLE);

    let mid_point = (crossterm::terminal::size().unwrap().0 / 2) - (file.len() as u16 / 2);
    crossterm::execute!(
        std::io::stdout(),
        crossterm::cursor::MoveTo(mid_point as u16, 0)
    )
    .unwrap();

    print!("{}", file);

    let end_point =
        crossterm::terminal::size().unwrap().0 - modified_str.len() as u16 - (modified as u16);
    crossterm::execute!(std::io::stdout(), crossterm::cursor::MoveTo(end_point, 0)).unwrap();

    print!("{}", modified_str);

    crossterm::execute!(
        std::io::stdout(),
        crossterm::cursor::MoveTo(cursor.0, cursor.1),
        crossterm::style::SetBackgroundColor(crossterm::style::Color::Black),
        crossterm::style::SetForegroundColor(crossterm::style::Color::White),
        crossterm::cursor::Show
    )
    .unwrap();
}

fn draw_text(buffer: &Vec<String>, scroll: (u16, u16)) {
    let cursor = crossterm::cursor::position().unwrap();
    crossterm::execute!(
        std::io::stdout(),
        crossterm::cursor::MoveTo(0, 1),
        crossterm::cursor::Hide
    )
    .unwrap();

    for (i, line) in buffer.iter().enumerate() {
        if i >= scroll.1 as usize
            && i < scroll.1 as usize + crossterm::terminal::size().unwrap().1 as usize - 2
        {
            println!("{}", line);
        }
    }

    crossterm::execute!(
        std::io::stdout(),
        crossterm::cursor::MoveTo(cursor.0, cursor.1),
        crossterm::cursor::Show
    )
    .unwrap();
}

fn render(file: &str, modified: bool, buffer: &Vec<String>, scroll: (u16, u16)) {
    let cursor = crossterm::cursor::position().unwrap();
    print!(
        "{}",
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    );
    draw_title(file, modified);
    draw_text(buffer, scroll);
    crossterm::execute!(
        std::io::stdout(),
        crossterm::cursor::MoveTo(cursor.0, cursor.1)
    )
    .unwrap();
}

fn main() {
    let args = Args::parse();

    let mut file = args.file;
    let mut modified = false;
    let mut text_cursor: (u16, u16) = (0, 0);
    let mut scroll: (u16, u16) = (0, 0);
    let mut normal_mode = true;

    // // see if the file exists and open it if it does
    // if !std::path::Path::new(&file).exists() {
    //     // if it doesn't, create it
    //     std::fs::File::create(&file).unwrap();
    // }

    // let initial_buffer = vec![String::new()];
    // let mut buffer: Vec<String> = initial_buffer.clone();

    let initial_buffer = if std::path::Path::new(&file).exists() {
        let string = std::fs::read_to_string(&file).unwrap().replace("\r\n", "\n");
        let mut vec = string.lines().map(|s| s.to_string()).collect::<Vec<String>>();
        if Some('\n') == string.chars().nth(string.len() - 1) {
            vec.push(String::new());
        }
        vec
    } else {
        vec![String::new()]
    };
    let mut buffer = initial_buffer.clone();

    setup();
    render(&file, modified, &buffer, scroll);

    crossterm::execute!(
        std::io::stdout(),
        crossterm::cursor::MoveTo(0, 1),
        crossterm::cursor::Show
    )
    .unwrap();

    loop {
        let mut needs_redraw = false;
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::FocusGained => println!("FocusGained"),
            crossterm::event::Event::FocusLost => println!("FocusLost"),
            crossterm::event::Event::Key(event) => {
                if event.kind != crossterm::event::KeyEventKind::Release {
                    match event.code {
                        crossterm::event::KeyCode::Char(c) => {
                            if event.modifiers == crossterm::event::KeyModifiers::CONTROL {
                                match c {
                                    's' => {
                                        needs_redraw = true;
                                    }
                                    'x' => {
                                        exit();
                                        std::process::exit(0);
                                    }
                                    'l' => {
                                        normal_mode = !normal_mode;
                                    }
                                    _ => {}
                                }
                            } else {
                                if normal_mode {
                                    // buffer[text_cursor.1 as usize].insert(text_cursor.0 as usize, c);
                                    // set the char at the cursor to c if it exists
                                    if text_cursor.0 < buffer[text_cursor.1 as usize].len() as u16 {
                                        buffer[text_cursor.1 as usize]
                                            .remove(text_cursor.0 as usize);
                                        buffer[text_cursor.1 as usize]
                                            .insert(text_cursor.0 as usize, c);
                                    } else {
                                        buffer[text_cursor.1 as usize].push(c);
                                    }
                                    print!("{}", c);
                                    std::io::stdout().flush().unwrap();
                                    text_cursor.0 += 1;
                                } else {
                                    buffer[text_cursor.1 as usize]
                                        .insert(text_cursor.0 as usize, c);
                                    let remaining = buffer[text_cursor.1 as usize]
                                        .split_off(text_cursor.0 as usize + 1);
                                    print!("{}{}", c, remaining);
                                    std::io::stdout().flush().unwrap();
                                    text_cursor.0 += 1;
                                    crossterm::execute!(
                                        std::io::stdout(),
                                        crossterm::cursor::MoveTo(
                                            text_cursor.0 - scroll.0,
                                            text_cursor.1 + 1 - scroll.1
                                        ),
                                    )
                                    .unwrap();
                                }
                            }
                        }
                        crossterm::event::KeyCode::Backspace => {
                            if text_cursor.0 > 0 {
                                if text_cursor.0 != buffer[text_cursor.1 as usize].len() as u16 {
                                    if normal_mode {
                                        buffer[text_cursor.1 as usize]
                                            .remove(text_cursor.0 as usize - 1);
                                        buffer[text_cursor.1 as usize]
                                            .insert(text_cursor.0 as usize - 1, ' ');
                                        text_cursor.0 -= 1;
                                        print!("\x08 \x08");
                                        std::io::stdout().flush().unwrap();
                                    }
                                } else {
                                    buffer[text_cursor.1 as usize].pop();
                                    text_cursor.0 -= 1;
                                    print!("\x08 \x08");
                                    std::io::stdout().flush().unwrap();
                                }
                            } else if text_cursor.1 > 0 {
                                text_cursor.1 -= 1;
                                text_cursor.0 = buffer[text_cursor.1 as usize].len() as u16;
                                crossterm::execute!(
                                    std::io::stdout(),
                                    crossterm::cursor::MoveTo(
                                        text_cursor.0 - scroll.0,
                                        text_cursor.1 + 1 - scroll.1
                                    ),
                                )
                                .unwrap();
                            }
                        }
                        crossterm::event::KeyCode::Enter => {
                            print!("\n");
                            text_cursor.1 += 1;
                            buffer.insert(text_cursor.1 as usize, String::new());
                            text_cursor.0 = 0;
                            modified = true;
                        }
                        crossterm::event::KeyCode::Left => {
                            if text_cursor.0 > 0 {
                                text_cursor.0 -= 1;
                                crossterm::execute!(
                                    std::io::stdout(),
                                    crossterm::cursor::MoveLeft(1)
                                )
                                .unwrap();
                            } else if text_cursor.1 > 0 {
                                text_cursor.1 -= 1;
                                text_cursor.0 = buffer[text_cursor.1 as usize].len() as u16;
                                crossterm::execute!(
                                    std::io::stdout(),
                                    crossterm::cursor::MoveTo(
                                        text_cursor.0 - scroll.0,
                                        text_cursor.1 + 1 - scroll.1
                                    ),
                                )
                                .unwrap();
                            }
                        }
                        crossterm::event::KeyCode::Right => {
                            if text_cursor.0 < buffer[text_cursor.1 as usize].len() as u16 {
                                text_cursor.0 += 1;
                                crossterm::execute!(
                                    std::io::stdout(),
                                    crossterm::cursor::MoveRight(1)
                                )
                                .unwrap();
                            } else if text_cursor.1 < buffer.len() as u16 - 1 {
                                text_cursor.1 += 1;
                                text_cursor.0 = 0;
                                crossterm::execute!(
                                    std::io::stdout(),
                                    crossterm::cursor::MoveTo(
                                        text_cursor.0 - scroll.0,
                                        text_cursor.1 + 1 - scroll.1
                                    ),
                                )
                                .unwrap();
                            }
                        }
                        crossterm::event::KeyCode::Up => {
                            if text_cursor.1 > 0 {
                                text_cursor.1 -= 1;
                                if text_cursor.0 > buffer[text_cursor.1 as usize].len() as u16 {
                                    text_cursor.0 = buffer[text_cursor.1 as usize].len() as u16;
                                }
                                crossterm::execute!(
                                    std::io::stdout(),
                                    crossterm::cursor::MoveTo(
                                        text_cursor.0 - scroll.0,
                                        text_cursor.1 + 1 - scroll.1
                                    ),
                                )
                                .unwrap();
                            }
                        }
                        crossterm::event::KeyCode::Down => {
                            if text_cursor.1 < buffer.len() as u16 - 1 {
                                text_cursor.1 += 1;
                                if text_cursor.0 > buffer[text_cursor.1 as usize].len() as u16 {
                                    text_cursor.0 = buffer[text_cursor.1 as usize].len() as u16;
                                }
                                crossterm::execute!(
                                    std::io::stdout(),
                                    crossterm::cursor::MoveTo(
                                        text_cursor.0 - scroll.0,
                                        text_cursor.1 + 1 - scroll.1
                                    ),
                                )
                                .unwrap();
                            }
                        }
                        _ => {}
                    }
                }
            }
            crossterm::event::Event::Mouse(event) => println!("{:?}", event),
            crossterm::event::Event::Paste(data) => println!("{:?}", data),
            crossterm::event::Event::Resize(_, _) => {
                needs_redraw = true;
            }
        }

        let is_modified = initial_buffer != buffer;
        if modified != is_modified {
            modified = is_modified;
            needs_redraw = true;
        }

        if needs_redraw {
            render(&file, modified, &buffer, scroll);
        }
    }
}
