use array_macro::array;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::io::Stdout;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use std::{io, thread};
use tui::backend::CrosstermBackend;
use tui::layout::Constraint;
use tui::style::{Color, Style};
use tui::widgets::{Row, Table, Cell};
use tui::Terminal;

mod cells;

const GEN_TIME: u64 = 5;
fn tear_down(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), io::Error> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn map_cells_to_rows(cells: Vec<cells::Cell>) -> Vec<Row<'static>> {
    let mut cell_rows: Vec<Row> = vec![];
    let mut current_row: Vec<Cell> = vec![];
    for i in 0..cells.len() {
        let cell = cells.get(i).unwrap();
        // current_row.push(cell.id.to_string());
        current_row.push(match cell.is_alive {
            true => Cell::from("").style(Style::default().bg(Color::LightGreen)),
            false => Cell::from("").style(Style::default().bg(Color::DarkGray)),
        });
        if cell.id != 0 && cell.id % cells::BOARD_WIDTH == 0  {
            let new_row = Row::new(current_row);
            cell_rows.push(new_row);
            current_row = vec![];
        }
    }
    cell_rows
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut cell_list = cells::init_vec();

    let max_iter = 500;
    let mut cur_iter = 0;
    let widths = array![Constraint::Length(1); cells::BOARD_WIDTH as usize];

    while cur_iter < max_iter {
        cell_list = cells::handle_generation_change(cell_list);
        terminal.draw(|f| {
            let rows = map_cells_to_rows(cell_list.clone());
            let table_block = Table::new(rows)
                .style(Style::default().fg(Color::White))
                .widths(&widths);
            f.render_widget(table_block, f.size());
        })?;
        thread::sleep(Duration::from_millis(GEN_TIME));
        cur_iter += 1;
    }
    tear_down(&mut terminal)?;
    Ok(())
}
