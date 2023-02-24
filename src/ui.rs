use crate::game::{
    cell, hard_drop_pos, tetromino::Tetromino, Game, FIELD_HEIGHT, FIELD_WIDTH,
    NEXT_TETROMINOES_SIZE,
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

fn get_cell_attribute(kind: cell::Kind) -> (&'static str, Style) {
    let style = Style::default().bg(BG_COLOR_TABLE[kind]);
    match kind {
        cell::NONE => ("  ", style),
        cell::GHOST => ("[]", style),
        _ => ("__", style),
    }
}

struct FieldWidget<'a> {
    game: &'a Game,
}

impl<'a> FieldWidget<'a> {
    fn new(game: &Game) -> FieldWidget {
        FieldWidget { game }
    }

    pub fn calc_coordinate(area: Rect, x: usize, y: usize) -> (u16, u16) {
        let px = area.x + (x * 2 - 2) as u16;
        let py = area.y + y as u16;

        (px, py)
    }
}

impl<'a> Widget for FieldWidget<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let game = &self.game;

        // フィールド
        for y in 0..FIELD_HEIGHT - 1 {
            for x in 1..FIELD_WIDTH - 1 {
                let (px, py) = Self::calc_coordinate(area, x, y);
                let (s, style) = get_cell_attribute(game.field[y][x] as usize);
                buf.set_string(px, py, s, style);
            }
        }

        let shape = game.tetromino.get_shape();
        let ghost_pos = hard_drop_pos(&game.field, &game.pos, &game.tetromino);

        // ゴースト
        for y in 0..4 {
            for x in 0..4 {
                if shape[y][x] != cell::NONE {
                    let (px, py) = Self::calc_coordinate(area, ghost_pos.x + x, ghost_pos.y + y);
                    let s = "[]";
                    let style = Style::default().fg(BG_COLOR_TABLE[game.tetromino.cell_kind()]);
                    buf.set_string(px, py, s, style);
                }
            }
        }

        // テトリミノ
        for y in 0..4 {
            for x in 0..4 {
                if shape[y][x] != cell::NONE {
                    let (px, py) = Self::calc_coordinate(area, game.pos.x + x, game.pos.y + y);
                    let (s, style) = get_cell_attribute(shape[y][x] as usize);
                    buf.set_string(px, py, s, style);
                }
            }
        }
    }
}

struct HoldTetrominoWidget<'a> {
    block: Option<Block<'a>>,
    tetromino: &'a Option<Tetromino>,
}

impl<'a> HoldTetrominoWidget<'a> {
    fn new(tetromino: &Option<Tetromino>) -> HoldTetrominoWidget {
        HoldTetrominoWidget {
            block: None,
            tetromino,
        }
    }

    fn block(mut self, block: Block<'a>) -> HoldTetrominoWidget<'a> {
        self.block = Some(block);
        self
    }
}

impl<'a> Widget for HoldTetrominoWidget<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let area = match self.block {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        let shape = match self.tetromino {
            Some(tetromino) => tetromino.get_shape(),
            None => Default::default(),
        };
        for y in 0..4 {
            for x in 0..4 {
                let px = area.x + (x * 2) as u16;
                let py = area.y + y as u16;
                let (s, style) = get_cell_attribute(shape[y][x]);
                buf.set_string(px, py, s, style);
            }
        }
    }
}

struct NextTetrominoesWidget<'a> {
    block: Option<Block<'a>>,
    next_tetrominoes: &'a VecDeque<Tetromino>,
}

impl<'a> NextTetrominoesWidget<'a> {
    fn new(next_tetrominoes: &VecDeque<Tetromino>) -> NextTetrominoesWidget {
        NextTetrominoesWidget {
            block: None,
            next_tetrominoes,
        }
    }

    fn block(mut self, block: Block<'a>) -> NextTetrominoesWidget<'a> {
        self.block = Some(block);
        self
    }
}

impl<'a> Widget for NextTetrominoesWidget<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let area = match self.block {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        for (i, tetromino) in self
            .next_tetrominoes
            .iter()
            .take(NEXT_TETROMINOES_SIZE)
            .enumerate()
        {
            let shape = tetromino.get_shape();
            for y in 0..4 {
                for x in 0..4 {
                    let px = area.x + (x * 2) as u16;
                    let py = area.y + (i * 4 + y) as u16;
                    let (s, style) = get_cell_attribute(shape[y][x]);
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
    let field = FieldWidget::new(&game);
    let box_border = Block::default()
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center);
    let hold_tetromino_box =
        HoldTetrominoWidget::new(&game.hold_tetromino).block(box_border.clone().title("HOLD"));
    let next_tetrominoes_box =
        NextTetrominoesWidget::new(&game.next_tetrominoes).block(box_border.title("NEXT"));
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
    f.render_widget(hold_tetromino_box, layout.left_pane_chunks[0]);
    f.render_widget(score_box, layout.left_pane_chunks[1]);
    f.render_widget(next_tetrominoes_box, layout.right_pane_chunks[0]);
}
