mod tetris;

use tetris::GameLoop;

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Rect, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::{stdout, Result};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut game = GameLoop::new();

    let mut text_a3 = String::new();
    let mut text_a4 = String::new();
    let time = std::time::SystemTime::now();

    let area0: Rect = Rect::new(1, 0, 18, 1);
    let area1: Rect = Rect::new(2, 2, 18, 1);
    let area2: Rect = Rect::new(1, 4, 20, 20);
    let area3: Rect = Rect::new(1, 25, 20, 1);
    let area4: Rect = Rect::new(23, 23, 10, 1);

    let mut counter = 0;
    loop {
        let size = terminal.size()?;

        //print!("{} {}, ", size.height, size.width);
        //print!("{} ", i[0][0]);

        game.tick();
        //text_a3 = counter.to_string();
        text_a3 = game.get_debug_value();
        text_a4 = time.elapsed().unwrap().as_secs().to_string();

        terminal.draw(|frame| {
            frame.render_widget(Paragraph::new("press 'q' to quit").white().on_blue(), area0);
            frame.render_widget(Paragraph::new(" T  E  T  R  I  S").white().on_red(), area1);

            frame.render_widget(
                Paragraph::new(game.map_to_string()).yellow().on_dark_gray(),
                area2,
            );
            frame.render_widget(Paragraph::new(text_a3).white().on_red(), area3);
            frame.render_widget(Paragraph::new(text_a4).white().on_red(), area4);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if key.code == KeyCode::Char('q') {
                        break;
                    }
                    game.input(key.code);
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

/*
counter += 1;
if counter % 100 == 0 {
    for i in 0..20 {
        for j in 0..10 {
            game.arr[i][j] = " ";
        }
    }
}
game.arr[2 * counter / 10 % 20][counter / 11 % 10] = "█";
game.arr[2 * counter / 10 % 20][counter / 13 % 10] = "█";
*/
