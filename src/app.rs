use crate::errors::{AppError, Result};
use crate::weather::api::{get_coordinates, get_forecast};
use crate::weather::models::Forecast;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, Clear, Padding, Paragraph, Row, Table};
use ratatui::{Frame, Terminal};
use std::fmt::format;
use std::io::Stdout;
use std::time::Duration;

pub struct App {
    pub state: AppState,
    pub city: String,
    pub forecast: Option<Forecast>,
    pub error: Option<String>,
    pub current_day: usize,
}

#[derive(PartialEq)]
pub enum AppState {
    Running,
    Loading,
    AskCity,
    Quit,
}

impl App {
    pub fn new() -> App {
        App {
            city: "".to_string(),
            forecast: None,
            state: AppState::AskCity,
            error: None,
            current_day: 0,
        }
    }

    pub async fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        while self.state != AppState::Quit {
            terminal.draw(|f| self.draw(f))?;

            if self.state == AppState::Loading {
                match self.fetch_forecast().await {
                    Ok(_) => self.state = AppState::Running,
                    Err(e) => {
                        self.error = Some(e.to_string());
                        self.state = AppState::AskCity;
                    }
                }
            }

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    self.handle_input(key.code)
                }
            }
        }

        Ok(())
    }

    pub fn handle_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Enter => {
                self.error = None;
                self.state = AppState::Loading;
            }
            KeyCode::Backspace => {
                self.city.pop();
            }
            KeyCode::Char(c) => {
                self.city.push(c);
            }
            KeyCode::Esc => {
                self.state = AppState::Quit;
            }
            KeyCode::Left => {
                if self.current_day > 0 {
                    self.current_day -= 1;
                }
            }
            KeyCode::Right => {
                if self.current_day < 6 {
                    self.current_day += 1;
                }
            }

            _ => {}
        }
    }

    async fn fetch_forecast(&mut self) -> Result<()> {
        let geo_api_res = get_coordinates(&self.city.as_str()).await?;
        let coords = &geo_api_res
            .results
            .ok_or(AppError::CityNotFound(self.city.clone()))?
            .into_iter()
            .next()
            .ok_or(AppError::CityNotFound(self.city.clone()))?;

        self.forecast = Some(get_forecast(&coords.latitude, &coords.longitude).await?);

        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) {
        match self.state {
            AppState::AskCity | AppState::Loading => self.draw_city_input_screen(frame),
            AppState::Running => self.draw_forecast(frame),
            AppState::Quit => {}
        }
    }

    fn draw_city_input_screen(&self, frame: &mut Frame) {
        let popup_area = Self::centered_rect(30, 25, frame.area());

        let block = Block::default()
            .title(" ⛅  Weather TUI ")
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center)
            .padding(Padding::uniform(1));

        let error = match self.error {
            Some(ref e) => Span::styled(e, Style::default().fg(Color::Red)),
            None => Span::raw(""),
        };

        let lines = vec![
            Line::from("Enter city:"),
            Line::from(format!("{}_", self.city)),
            Line::from(""),
            Line::from(error),
        ];

        let paragraph = Paragraph::new(lines)
            .block(block)
            .alignment(Alignment::Center);

        frame.render_widget(Clear, popup_area);
        frame.render_widget(paragraph, popup_area);
    }

    fn draw_forecast(&self, frame: &mut Frame) {
        let Some(forecast) = self.forecast.as_ref() else {
            return;
        };

        let hours = &forecast.hourly;
        let start = self.current_day * 24;
        let end = start + 24;

        let date = &hours.time[start][..10]; // 2026-04-04
        let title = format!(" {} -- dia {}/7 ", date, self.current_day + 1);

        let block = Block::default()
            .borders(Borders::ALL)
            .title(title)
            .title_alignment(Alignment::Center);

        let widths = [
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ];

        let header = Row::new(["Hour", "Temp", "Wind Speed", "Weather"])
            .style(Style::new().bold())
            .bottom_margin(1);

        let rows: Vec<Row> = (start..end)
            .map(|i| {
                Row::new([
                    Cell::from(hours.time[i].clone()[11..16].to_string()),
                    Cell::from(format!(
                        "{:4} {}",
                        hours.temperature[i], forecast.hourly_units.temperature
                    )),
                    Cell::from(format!(
                        "{:4} {}",
                        hours.wind_speed[i], forecast.hourly_units.wind_speed
                    )),
                    Cell::from(hours.weather_code[i].to_string()),
                ])
            })
            .collect();

        let table = Table::new(rows, widths).header(header).column_spacing(1).block(block);

        frame.render_widget(table, frame.area());
    }

    fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(vertical[1])[1]
    }
}
