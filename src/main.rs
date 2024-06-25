mod errors;
mod installer;
mod step;
mod tui;
mod utils;

use installer::*;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

fn main() -> Result<()> {
    errors::install_hooks()?;
    let mut terminal = tui::init()?;
    App::default().run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit()?,
            KeyCode::Char('i') => install_vimcat()?,
            _ => {}
        }
        Ok(())
    }

    fn exit(&mut self) -> Result<()> {
        self.exit = true;
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Vimcat CLI ".bold());
        let instructions = Title::from(Line::from(vec![
            " Install ".into(),
            "<I>".blue().bold(),
            " Quit ".into(),
            "<Q>".blue().bold(),
        ]));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let text = Text::from(vec![Line::from(vec![
            "Welcome to vimcat cli".into(),
            step::get_current_name()
                .unwrap_or("unknown step".to_string())
                .white(),
            step::get_id().to_string().yellow(),
        ])]);

        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

fn install_vimcat() -> Result<()> {
    let steps: &[(&str, fn() -> Result<()>)] = &[
        ("cleanning configs", cleanup_configs),
        ("download configs", download_configs),
        ("installing homebrew", install_homebrew),
        ("installing git", install_git),
        ("installing fzf", install_fzf),
        ("installing nerdfonts", install_nerdfonts),
        ("installing iterm", install_iterm),
        ("setting up item", setup_iterm),
        ("installing tmux", install_tmux),
        ("installing tpm", install_tpm),
        ("setting up tmux", setup_tmux),
        ("installing ohmyzsh", install_ohmyzsh),
        ("setting up ohmyzsh", setup_ohmyzsh),
        ("installing zshsyntax", install_zshsyntax),
        ("setting up zshsyntax", setup_zshsyntax),
        ("installing zsh authsuggestions", install_zshautosuggestions),
        ("installing powerlevel10k", install_powerlevel10k),
        ("setting up powerlevel10k", setup_powerlevel10k),
        ("installing neovim", install_neovim),
        ("setting up neovim", setup_neovim),
        ("cleanning configs", cleanup_configs),
    ];

    for (step_name, step_fn) in steps {
        step::run_step(step_name.to_string(), *step_fn)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    // ANCHOR: render test
    use super::*;

    #[test]
    fn render() {
        let app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
            "┃                  Vimcat CLI                    ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);
        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(14, 0, 22, 1), title_style);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

        assert_eq!(buf, expected);
    }

    #[test]
    fn handle_key_event() -> io::Result<()> {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Right.into());
        assert_eq!(app.counter, 1);

        app.handle_key_event(KeyCode::Left.into());
        assert_eq!(app.counter, 0);

        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert_eq!(app.exit, true);

        Ok(())
    }
}
