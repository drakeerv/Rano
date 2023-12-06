use std::io::Write;

fn setup() {
    crossterm::terminal::enable_raw_mode().ok();
    crossterm::execute!(
        std::io::stdout(),
        crossterm::terminal::SetTitle("Rano"),
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

    crossterm::execute!(
        std::io::stdout(),
        crossterm::cursor::MoveTo(0, 0),
    ).unwrap();

    print!("Rano");

    let mid_point = (crossterm::terminal::size().unwrap().0 / 2) - (file.len() as u16 / 2);
    crossterm::execute!(
        std::io::stdout(),
        crossterm::cursor::MoveTo(mid_point as u16, 0)
    ).unwrap();

    print!("{}", file);

    let end_point = crossterm::terminal::size().unwrap().0 - 1 - (modified as u16);
    crossterm::execute!(
        std::io::stdout(),
        crossterm::cursor::MoveTo(end_point, 0)
    ).unwrap();

    print!("*");

    crossterm::execute!(
        std::io::stdout(),
        crossterm::cursor::MoveTo(cursor.0, cursor.1),
        crossterm::style::SetBackgroundColor(crossterm::style::Color::Black),
        crossterm::style::SetForegroundColor(crossterm::style::Color::White),
        crossterm::cursor::Show
    )
    .unwrap();
}

fn render(file: &str, modified: bool) {
    let cursor = crossterm::cursor::position().unwrap();
    draw_title(file, modified);
    crossterm::execute!(
        std::io::stdout(),
        crossterm::cursor::MoveTo(cursor.0, cursor.1)
    )
    .unwrap();
}

fn main() {
    let mut file = "New Buffer";
    let mut modified = false;
    let mut text_cursor: (u16, u16) = (0, 0);
    let mut scroll: (u16, u16) = (0, 0);

    setup();
    render(file, modified);

    let mut buffer: Vec<String> = Vec::new();
    buffer.push(String::new());

    crossterm::execute!(
        std::io::stdout(),
        crossterm::cursor::MoveTo(0, 1),
        crossterm::cursor::Show
    ).unwrap();

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
                                if c == 's' {
                                    needs_redraw = true;
                                } else if c == 'x' {
                                    exit();
                                    std::process::exit(0);
                                }
                            } else {
                                buffer[text_cursor.1 as usize].insert(text_cursor.0 as usize, c);
                                print!("{}", c);
                                std::io::stdout().flush().unwrap();
                                text_cursor.0 += 1;
                            }
                        }
                        crossterm::event::KeyCode::Backspace => {
                            if text_cursor.0 > 0 {
                                if text_cursor.0 != buffer[text_cursor.1 as usize].len() as u16 {
                                    buffer[text_cursor.1 as usize].remove(text_cursor.0 as usize - 1);
                                    text_cursor.0 -= 1;
                                    print!("\x08 \x08");
                                    std::io::stdout().flush().unwrap();
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
                                    crossterm::cursor::MoveTo(text_cursor.0 - scroll.0, text_cursor.1 + 1 - scroll.1),
                                ).unwrap();
                            }
                        }
                        crossterm::event::KeyCode::Enter => {
                            println!("");
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
                                ).unwrap();
                            }
                        }
                        crossterm::event::KeyCode::Right => {
                            if text_cursor.0 < buffer[text_cursor.1 as usize].len() as u16 {
                                text_cursor.0 += 1;
                                crossterm::execute!(
                                    std::io::stdout(),
                                    crossterm::cursor::MoveRight(1)
                                ).unwrap();
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
                                    crossterm::cursor::MoveUp(1)
                                ).unwrap();
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
                                    crossterm::cursor::MoveDown(1)
                                ).unwrap();
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

        if needs_redraw {
            render(file, modified);
        }
    }
}
