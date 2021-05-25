use termion::input::TermRead;
use termion::event::Event;
use std::io::{Write, stdout, stdin};
use termion::raw::IntoRawMode;
use termion::cursor::DetectCursorPos;
use enigo::Enigo;
use enigo::KeyboardControllable;

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();
    let stdin = stdin.lock();

    let mut enigo = Enigo::new();

    let mut alt_tabbing = false;
    let mut escaping = false;

    for e in stdin.events() {
        let e = e.unwrap();
        match e {
            Event::Key(key) => {
                let key = key;
                let pos = stdout.cursor_pos().unwrap();
                write!(stdout, "{:?}{}\n", key, termion::cursor::Goto(1, pos.1)).unwrap();
                if let termion::event::Key::Ctrl('c') = key {
                    if escaping {
                        break
                    }
                    escaping = true;
                }
                else {
                    escaping = false;
                }
                match key {
                    termion::event::Key::Backspace => {enigo.key_click(enigo::Key::Backspace)}
                    termion::event::Key::Left => {enigo.key_click(enigo::Key::LeftArrow)}
                    termion::event::Key::Right => {enigo.key_click(enigo::Key::RightArrow)}
                    termion::event::Key::Up => {enigo.key_click(enigo::Key::UpArrow)}
                    termion::event::Key::Down => {enigo.key_click(enigo::Key::DownArrow)}
                    termion::event::Key::Home => {}
                    termion::event::Key::End => {}
                    termion::event::Key::PageUp => {}
                    termion::event::Key::PageDown => {}
                    termion::event::Key::BackTab => {
                        enigo.key_down(enigo::Key::Shift);
                        enigo.key_click(enigo::Key::Tab);
                        enigo.key_up(enigo::Key::Shift);
                    }
                    termion::event::Key::Char('\t') => {
                        enigo.key_click(enigo::Key::Tab);
                    }
                    termion::event::Key::Delete => {}
                    termion::event::Key::Insert => {}
                    termion::event::Key::F(_) => {}
                    termion::event::Key::Char('\n') => {
                        if alt_tabbing {
                            enigo.key_up(enigo::Key::Control);
                            alt_tabbing = false;
                        }
                        else {
                            enigo.key_click(enigo::Key::Return);
                        }
                    }
                    termion::event::Key::Char(key) => {
                        if key.is_uppercase() {
                            enigo.key_down(enigo::Key::Shift);
                            enigo.key_click(enigo::Key::Layout(key));
                            enigo.key_up(enigo::Key::Shift);
                        }
                        else {
                            enigo.key_click(enigo::Key::Layout(key));
                        }
                    }
                    termion::event::Key::Alt('{') => {
                        enigo.key_down(enigo::Key::Meta);
                        enigo.key_click(enigo::Key::LeftArrow);
                        enigo.key_up(enigo::Key::Meta);
                    }
                    termion::event::Key::Alt('}') => {
                        enigo.key_down(enigo::Key::Meta);
                        enigo.key_click(enigo::Key::RightArrow);
                        enigo.key_up(enigo::Key::Meta);
                    }
                    termion::event::Key::Alt('\t') => {
                        if !alt_tabbing {
                            enigo.key_down(enigo::Key::Control);
                        }
                        enigo.key_click(enigo::Key::Tab);
                        alt_tabbing = true;
                    }
                    termion::event::Key::Alt(key) => {
                        enigo.key_down(enigo::Key::Control);
                        enigo.key_click(enigo::Key::Layout(key));
                        enigo.key_up(enigo::Key::Control);
                    }
                    termion::event::Key::Ctrl(key) => {
                        enigo.key_down(enigo::Key::Meta);
                        enigo.key_click(enigo::Key::Layout(key));
                        enigo.key_up(enigo::Key::Meta);
                    }
                    termion::event::Key::Null => {}
                    termion::event::Key::Esc => {enigo.key_click(enigo::Key::Escape)}
                    termion::event::Key::__IsNotComplete => {}
                }
            }
            Event::Mouse(_) => {}
            _ => {}
        }
    }
    if alt_tabbing {
        enigo.key_up(enigo::Key::Control);
    }
}
