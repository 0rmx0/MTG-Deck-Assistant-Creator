use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};


use crate::app::App;
use crate::theme::Theme;

fn block(title: &str) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Theme::BG_ELEVATED))
        .title(Line::from(Span::styled(
            format!(" {title} "),
            Style::default()
                .fg(Theme::CORAL)
                .add_modifier(Modifier::BOLD),
        )))
        .style(Style::default().bg(Theme::BG_SECONDARY))
}

pub fn draw(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(f.area());

    draw_header(f, app, chunks[0]);
    draw_body(f, app, chunks[1]);
    draw_status(f, app, chunks[2]);
}

fn draw_header(f: &mut ratatui::Frame, app: &App, area: Rect) {
    let header = block("HELLFIRE DECKBUILDER");
    let inner = header.inner(area);
    f.render_widget(header, area);

    let left = Paragraph::new(vec![
        Line::from(Span::styled(
            "MASQUE COUSU · LE DECK EST UNE ARME",
            Style::default().fg(Theme::TEXT_PRIMARY),
        )),
        Line::from(Span::styled(
            format!("Format : {}", app.deck.format.label()),
            Style::default().fg(Theme::CYAN),
        )),
        Line::from(Span::styled(
            format!("{} cartes recrutées", app.draw_stats().total_cards),
            Style::default().fg(Theme::TEXT_SECONDARY),
        )),
    ]);
    f.render_widget(left, inner);
}

fn draw_body(f: &mut ratatui::Frame, app: &mut App, area: Rect) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    draw_deck_list(f, app, cols[0]);
    draw_right_pane(f, app, cols[1]);
}

fn draw_deck_list(f: &mut ratatui::Frame, app: &App, area: Rect) {
    let b = block("TON ESCOUADE");
    let inner = b.inner(area);
    f.render_widget(b, area);

    let items: Vec<ListItem> = app
        .deck
        .entries
        .iter()
        .map(|e| {
            let qty = Span::styled(
                format!("{:>2}x ", e.quantity),
                Style::default().fg(Theme::EMBER),
            );
            let name = Span::styled(
                e.card.name.clone(),
                Style::default().fg(Theme::TEXT_PRIMARY),
            );
            let cost = Span::styled(
                format!(" {}", e.card.mana_cost.as_ref().map(|m| m.to_notation()).unwrap_or_default()),
                Style::default().fg(Theme::TEXT_SECONDARY),
            );
            ListItem::new(Line::from(vec![qty, name, cost]))
        })
        .collect();

    let list = List::new(items)
        .highlight_style(Style::default().fg(Theme::CORAL))
        .highlight_symbol(">> ");
    f.render_stateful_widget(list, inner, &mut ListState::default());
}

fn draw_right_pane(f: &mut ratatui::Frame, app: &mut App, area: Rect) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Min(5)])
        .split(area);

    draw_curve(f, app, rows[0]);
    draw_actions(f, rows[1]);
}

fn draw_curve(f: &mut ratatui::Frame, app: &App, area: Rect) {
    let b = block("COURBE DE MANA");
    let inner = b.inner(area);
    f.render_widget(b, area);

    let stats = app.draw_stats();
    let max = stats.curve.max().max(1) as u64;
    let mut bars = Vec::new();
    for cmc in 0..=7u32 {
        let label = if cmc == 7 { "7+".into() } else { cmc.to_string() };
        bars.push((" ".to_string(), label, stats.curve.get(cmc) as u64, max));
    }

    let mut x = inner.left();
    let width = 3;
    for (bar, label, value, max) in bars {
        let height = inner.height;
        let filled = ((value as f32 / max as f32) * height as f32) as u16;
        for i in 0..height {
            let y = inner.bottom() - 1 - i;
            let ch = if i < filled { "█" } else { "░" };
            let color = if i < filled { Theme::CORAL } else { Theme::BG_ELEVATED };
            f.buffer_mut()
                .set_string(x, y, ch, Style::default().fg(color).bg(Theme::BG_SECONDARY));
        }
        let _ = bar;
        f.buffer_mut().set_string(
            x,
            inner.bottom(),
            &label,
            Style::default().fg(Theme::TEXT_SECONDARY),
        );
        x += width;
        if x + width > inner.right() {
            break;
        }
    }
}

fn draw_actions(f: &mut ratatui::Frame, area: Rect) {
    let b = block("ACTIONS");
    let inner = b.inner(area);
    f.render_widget(b, area);

    let lines = vec![
        Line::from(vec![
            Span::styled("[R] ", Style::default().fg(Theme::CORAL)),
            Span::styled("Recruter une carte", Style::default().fg(Theme::TEXT_PRIMARY)),
        ]),
        Line::from(vec![
            Span::styled("[S] ", Style::default().fg(Theme::EMBER)),
            Span::styled("Sabotage (retirer une carte)", Style::default().fg(Theme::TEXT_PRIMARY)),
        ]),
        Line::from(vec![
            Span::styled("[V] ", Style::default().fg(Theme::CYAN)),
            Span::styled("Lever le poing (valider)", Style::default().fg(Theme::TEXT_PRIMARY)),
        ]),
        Line::from(vec![
            Span::styled("[Q] ", Style::default().fg(Theme::TEXT_SECONDARY)),
            Span::styled("Se replier (quitter)", Style::default().fg(Theme::TEXT_PRIMARY)),
        ]),
    ];
    f.render_widget(Paragraph::new(lines).wrap(Wrap { trim: true }), inner);
}

fn draw_status(f: &mut ratatui::Frame, app: &App, area: Rect) {
    let (color, symbol) = if app.status_is_error {
        (Theme::CORAL, "!!")
    } else if app.status.contains("validé") {
        (Theme::EMBER, "✔")
    } else {
        (Theme::CYAN, "»")
    };
    let line = Line::from(vec![
        Span::styled(format!("{symbol} "), Style::default().fg(color)),
        Span::styled(app.status.clone(), Style::default().fg(Theme::TEXT_PRIMARY)),
    ]);
    f.render_widget(
        Paragraph::new(line).style(Style::default().bg(Theme::BG_PRIMARY)),
        area,
    );
}

