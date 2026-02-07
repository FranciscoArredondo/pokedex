pub mod pokemon_detail;
pub mod pokemon_list;
pub mod team_builder;
pub mod type_chart;

use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Tabs};
use ratatui::Frame;

use crate::app::{App, Screen};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(f.area());

    draw_tabs(f, app, chunks[0]);

    match app.screen {
        Screen::PokemonList => pokemon_list::draw(f, app, chunks[1]),
        Screen::PokemonDetail => pokemon_detail::draw(f, app, chunks[1]),
        Screen::TypeChart => type_chart::draw(f, app, chunks[1]),
        Screen::TeamBuilder => team_builder::draw(f, app, chunks[1]),
    }

    // Error overlay
    if let Some(ref msg) = app.error_message {
        let area = centered_rect(60, 20, f.area());
        f.render_widget(Clear, area);
        let block = Block::default()
            .title(" Error ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Red));
        let text = Paragraph::new(format!("{}\n\nPress any key to dismiss", msg))
            .block(block)
            .style(Style::default().fg(Color::Red));
        f.render_widget(text, area);
    }
}

fn draw_tabs(f: &mut Frame, app: &App, area: Rect) {
    let titles: Vec<Span> = Screen::all()
        .iter()
        .map(|s| {
            let style = if *s == app.screen {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            Span::styled(format!(" {} ", s.label()), style)
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Pokémon TUI ")
                .title_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        )
        .select(app.screen.index())
        .highlight_style(Style::default().fg(Color::Yellow));
    f.render_widget(tabs, area);
}

pub fn type_color(type_name: &str) -> Color {
    match type_name {
        "normal" => Color::Rgb(168, 168, 120),
        "fire" => Color::Rgb(240, 128, 48),
        "water" => Color::Rgb(104, 144, 240),
        "electric" => Color::Rgb(248, 208, 48),
        "grass" => Color::Rgb(120, 200, 80),
        "ice" => Color::Rgb(152, 216, 216),
        "fighting" => Color::Rgb(192, 48, 40),
        "poison" => Color::Rgb(160, 64, 160),
        "ground" => Color::Rgb(224, 192, 104),
        "flying" => Color::Rgb(168, 144, 240),
        "psychic" => Color::Rgb(248, 88, 136),
        "bug" => Color::Rgb(168, 184, 32),
        "rock" => Color::Rgb(184, 160, 56),
        "ghost" => Color::Rgb(112, 88, 152),
        "dragon" => Color::Rgb(112, 56, 248),
        "dark" => Color::Rgb(112, 88, 72),
        "steel" => Color::Rgb(184, 184, 208),
        "fairy" => Color::Rgb(238, 153, 172),
        _ => Color::White,
    }
}

/// Helper to create a centered rect
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}
