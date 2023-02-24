use crate::block::BlockShape;
use crate::game::{
    cell, hard_drop_pos, FieldSize, Game, FIELD_HEIGHT, FIELD_WIDTH, NEXT_BLOCKS_SIZE,
};
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::collections::VecDeque;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Widget},
    Frame, Terminal,
};

const BG_COLOR_TABLE: [Color; 10] = [
    Color::Rgb(0, 0, 0),       // 何もなし
    Color::Rgb(127, 127, 127), // 壁
    Color::Rgb(0, 0, 0),       // ゴースト
    Color::Rgb(0, 255, 255),   // I
    Color::Rgb(255, 255, 0),   // O
    Color::Rgb(0, 255, 0),     // S
    Color::Rgb(255, 0, 0),     // Z
    Color::Rgb(0, 0, 255),     // J
    Color::Rgb(255, 127, 0),   // L
    Color::Rgb(255, 0, 255),   // T
];

fn get_block(kind: cell::Kind) -> (&'static str, Style) {
    let style = Style::default().bg(BG_COLOR_TABLE[kind]);
    match kind {
        cell::NONE => ("  ", style),
        cell::GHOST => ("[]", style),
        _ => ("__", style),
    }
}

struct FieldWidget<'a> {
    field: &'a FieldSize,
}

impl<'a> FieldWidget<'a> {
    fn new(field: &FieldSize) -> FieldWidget {
        FieldWidget { field }
    }
}

impl<'a> Widget for FieldWidget<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        for y in 0..FIELD_HEIGHT - 1 {
            for x in 1..FIELD_WIDTH - 1 {
                let px = area.x + (x * 2 - 2) as u16;
                let py = area.y + y as u16;
                if px < buf.area().x
                    || buf.area().x + buf.area.width <= px
                    || py < buf.area().y
                    || buf.area().y + buf.area.width <= py
                {
                    continue;
                }
                let (s, style) = get_block(self.field[y][x] as usize);
                buf.set_string(px, py, s, style);
            }
        }
    }
}

struct HoldBlockWidget<'a> {
    block: Option<Block<'a>>,
    hold: &'a Option<BlockShape>,
}

impl<'a> HoldBlockWidget<'a> {
    fn new(hold: &Option<BlockShape>) -> HoldBlockWidget {
        HoldBlockWidget { block: None, hold }
    }

    fn block(mut self, block: Block<'a>) -> HoldBlockWidget<'a> {
        self.block = Some(block);
        self
    }
}

impl<'a> Widget for HoldBlockWidget<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let area = match self.block {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        for y in 0..4 {
            for x in 0..4 {
                let px = area.x + (x * 2) as u16;
                let py = area.y + y as u16;
                match self.hold {
                    Some(hold) => {
                        let (s, style) = get_block(hold[y][x]);
                        buf.set_string(px, py, s, style);
                    }
                    None => buf.set_string(
                        px,
                        py,
                        "  ",
                        Style::default().bg(BG_COLOR_TABLE[cell::NONE]),
                    ),
                }
            }
        }
    }
}

struct NextBlockWidget<'a> {
    block: Option<Block<'a>>,
    next_blocks: &'a VecDeque<BlockShape>,
}

impl<'a> NextBlockWidget<'a> {
    fn new(next_blocks: &VecDeque<BlockShape>) -> NextBlockWidget {
        NextBlockWidget {
            block: None,
            next_blocks,
        }
    }

    fn block(mut self, block: Block<'a>) -> NextBlockWidget<'a> {
        self.block = Some(block);
        self
    }
}

impl<'a> Widget for NextBlockWidget<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let area = match self.block {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        for (i, block) in self.next_blocks.iter().take(NEXT_BLOCKS_SIZE).enumerate() {
            for y in 0..4 {
                for x in 0..4 {
                    let px = area.x + (x * 2) as u16;
                    let py = area.y + (i * 4 + y) as u16;
                    let (s, style) = get_block(block[y][x]);
                    buf.set_string(px, py, s, style);
                }
            }
        }
    }
}

struct GameLayout {
    pub center_pane_chunks: Vec<Rect>,
    pub left_pane_chunks: Vec<Rect>,
    pub right_pane_chunks: Vec<Rect>,
}

pub struct Ui {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    game_layout: GameLayout,
}

impl Ui {
    pub fn new() -> Result<Ui, Box<dyn Error>> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Ui {
            terminal,
            game_layout: create_game_layout(),
        })
    }

    pub fn shutdown(self) -> Result<(), Box<dyn Error>> {
        disable_raw_mode()?;
        let mut terminal = self.terminal;
        execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
        terminal.show_cursor()?;
        Ok(())
    }

    pub fn draw(&mut self, game: &Game) -> io::Result<()> {
        self.terminal
            .draw(|f| draw_game(f, &self.game_layout, &game))?;
        Ok(())
    }

    pub fn game_over(&mut self, game: &Game) -> io::Result<()> {
        loop {
            self.terminal.draw(|f| {
                draw_game(f, &self.game_layout, &game);
                let mut dialog_area = self.game_layout.center_pane_chunks[0].inner(&Margin {
                    vertical: 10,
                    horizontal: 5,
                });
                dialog_area.height = 3;

                let dialog = Paragraph::new("GAME  OVER")
                    .block(Block::default().borders(Borders::ALL))
                    .style(Style::default().fg(Color::White).bg(Color::Black))
                    .alignment(Alignment::Center);

                f.render_widget(tui::widgets::Clear, dialog_area);
                f.render_widget(dialog, dialog_area);
            })?;

            if let Event::Key(_) = event::read()? {
                break;
            }
        }

        Ok(())
    }
}

fn create_game_layout() -> GameLayout {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(10),
                Constraint::Length(24),
                Constraint::Length(10),
                Constraint::Percentage(100),
            ]
            .as_ref(),
        )
        .split(Rect::new(0, 0, 80, 24));
    let center_pane_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(24), Constraint::Percentage(100)].as_ref())
        .split(chunks[1]);
    let left_pane_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(6),
                Constraint::Length(3),
                Constraint::Percentage(100),
            ]
            .as_ref(),
        )
        .split(chunks[0]);
    let right_pane_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(14), Constraint::Percentage(100)].as_ref())
        .split(chunks[2]);

    GameLayout {
        center_pane_chunks,
        left_pane_chunks,
        right_pane_chunks,
    }
}

fn draw_game<B: Backend>(f: &mut Frame<B>, layout: &GameLayout, game: &Game) {
    let mut field_buf = game.field;

    let ghost_pos = hard_drop_pos(&game.field, &game.pos, &game.block);
    for y in 0..4 {
        for x in 0..4 {
            if game.block[y][x] != cell::NONE {
                field_buf[y + ghost_pos.y][x + ghost_pos.x] = cell::GHOST;
            }
        }
    }

    for y in 0..4 {
        for x in 0..4 {
            if game.block[y][x] != cell::NONE {
                field_buf[y + game.pos.y][x + game.pos.x] = game.block[y][x];
            }
        }
    }

    let field = FieldWidget::new(&field_buf);
    let box_border = Block::default()
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center);
    let hold_block_box = HoldBlockWidget::new(&game.hold).block(box_border.clone().title("HOLD"));
    let next_blocks_box = NextBlockWidget::new(&game.next_blocks).block(box_border.title("NEXT"));
    let score_box = Paragraph::new(game.score.to_string())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("SCORE")
                .title_alignment(Alignment::Center),
        )
        .style(Style::default())
        .alignment(Alignment::Right);

    f.render_widget(field, layout.center_pane_chunks[0]);
    f.render_widget(hold_block_box, layout.left_pane_chunks[0]);
    f.render_widget(score_box, layout.left_pane_chunks[1]);
    f.render_widget(next_blocks_box, layout.right_pane_chunks[0]);
}
