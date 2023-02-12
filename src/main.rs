use std::cmp;
use std::io;
use std::io::Stdout;
use crossterm::terminal;
use crossterm::event;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use crossterm::event::Event;
use crossterm::event::KeyEvent;
use crossterm::event::KeyCode;
use tui::Terminal;
use tui::Frame;
use tui::widgets::List;
use tui::widgets::ListItem;
use tui::style::Style;
use tui::style::Color;
use tui::backend::CrosstermBackend;
use crate::menu_item::MenuItem;
use crate::menu_item::MenuItemKind;
use crate::menu_item::MenuItemMeaning;

#[macro_use]
extern crate crossterm;

mod menu_item;

struct NmaintUi<'a> {
    is_running: bool,
    focus: usize,
    menu: [MenuItem<'a>; 4],
    term: Terminal<CrosstermBackend<Stdout>>,
}

impl<'a> NmaintUi<'a> {
    fn new() -> Result<Self, io::Error> {
        let backend = CrosstermBackend::new(io::stdout());
        let term = Terminal::new(backend)?;

        Ok(Self {
            is_running: false,
            focus: 0,
            menu: [
                MenuItem {
                    kind: MenuItemKind::Basic(&[]),
                    meaning: MenuItemMeaning::Update,
                },
                MenuItem {
                    kind: MenuItemKind::Basic(&[]),
                    meaning: MenuItemMeaning::Upgrade,
                },
                MenuItem {
                    kind: MenuItemKind::Basic(&[]),
                    meaning: MenuItemMeaning::Use,
                },
                MenuItem {
                    kind: MenuItemKind::Checkbox(false),
                    meaning: MenuItemMeaning::Update,
                },
            ],
            term
        })
    }

    fn start(&mut self) -> Result<(), io::Error> {
        self.is_running = true;

        terminal::enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen)?;

        while self.is_running {
            self.term.draw(self.render())?;

            match event::read().unwrap() {
                Event::Key(KeyEvent { code: key_code, .. }) => {
                    match key_code {
                        KeyCode::Char(pressed_char) => {
                            match pressed_char {
                                ' ' => self.select(),
                                'k' => self.focus = cmp::max((self.focus as isize) - 1, 0) as usize,
                                'j' => self.focus = cmp::min(self.focus + 1, self.menu.len() - 1),
                                'q' => self.is_running = false,
                                _ => (),
                            };
                        },
                        KeyCode::Enter => self.select(),
                        _ => (),
                    };
                },
                _ => (),
            };
        }

        terminal::disable_raw_mode()?;
        execute!(self.term.backend_mut(), LeaveAlternateScreen)?;

        Ok(())
    }

    fn render(&self) -> impl FnOnce(&mut Frame<CrosstermBackend<Stdout>>) {
        let list = List::new(self.menu.iter().enumerate().map(|(i, menu_item)| {
            let mut style = Style::default();

            if i == self.focus {
                style = style.bg(Color::Blue);
            }

            let title = format!("{}{}", match menu_item.kind {
                MenuItemKind::Checkbox(on) => format!("[{}] ", {
                    if on { '*' }
                    else { ' ' }
                }),
                _ => String::new(),
            }, menu_item.meaning);

            ListItem::new(title).style(style)
        }).collect::<Vec<ListItem>>());

        move |frame| {
            frame.render_widget(list, frame.size());
        }
    }

    fn select(&mut self) {
        match self.menu[self.focus].kind {
            MenuItemKind::Checkbox(ref mut on) => *on = !*on,
            _ => (),
        };
    }
}

fn main() -> Result<(), io::Error> {
    let mut ui = NmaintUi::new()?;
    ui.start()?;

    Ok(())
}

