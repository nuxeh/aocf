use crossterm::{
    cursor::{ Hide, Show},
    event::{
        self,
        Event,
        KeyEvent,
        KeyCode::*,
    },
    queue,
    terminal::{
        self,
        Clear,
        ClearType,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    style::{
        Color::*,
        Attribute::*,
    },
};
use std::io::{stderr, Write};
use failure::Error;
use termimad::{
    MadSkin, MadView, Alignment, Area, CompoundStyle,
};

fn view_area() -> Area {
    let mut area = Area::full_screen();
    area.pad_for_max_width(120);
    area
}

fn run_pretty(skin: MadSkin, content: &str) -> Result<(), Error> {
    let mut w = stderr();
    queue!(w, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    queue!(w, Hide)?;
    let mut view = MadView::from(content.to_owned(), view_area(), skin);
    loop {
        view.write_on(&mut w)?;
        w.flush()?;
        match event::read() {
            Ok(Event::Key(KeyEvent{code, ..})) => {
                match code {
                    Up => view.try_scroll_lines(-1),
                    Down => view.try_scroll_lines(1),
                    PageUp => view.try_scroll_pages(-1),
                    PageDown => view.try_scroll_pages(1),
                    _ => break,
                }
            }
            Ok(Event::Resize(..)) => {
                queue!(w, Clear(ClearType::All))?;
                view.resize(&view_area());
            }
            _ => {}
        }
    }
    terminal::disable_raw_mode()?;
    queue!(w, Show)?;
    queue!(w, LeaveAlternateScreen)?;
    w.flush()?;
    Ok(())
}

fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.table.align = Alignment::Center;
    skin.set_headers_fg(AnsiValue(178));
    skin.bold.set_fg(Yellow);
    skin.italic.overwrite_with(&CompoundStyle::with_attr(Bold));
    skin.italic.set_fg(White);
    skin.scrollbar.thumb.set_fg(AnsiValue(178));
    skin.code_block.align = Alignment::Center;
    skin.set_global_bg(Rgb{ r: 0x0f, g: 0x0f, b: 0x23 });
    //skin.inline_code.set_bg(Rgb{ r: 0x10, g: 0x10, b: 0x1a });
    skin
}

pub fn make_pretty(content: &str) -> Result<(), Error> {
    let skin = make_skin();
    run_pretty(skin, content)
}
