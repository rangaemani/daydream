use crate::app::AppState;
use core::fmt;
use ratatui::{
    prelude::*,
    widgets::{calendar::*, Block, BorderType, Borders},
};
use std::{collections::HashMap, rc::Rc};
use time::{Date, Month, OffsetDateTime};

/// Holds information about calendar events and holidays.
#[derive(Clone, PartialEq)]
pub struct CalendarInfo {
    /// Stores calendar events.
    pub events: CalendarEventStore,
    /// Maps dates to their holiday names.
    pub holidays: HashMap<Date, String>,
}

impl fmt::Display for CalendarInfo {
    /// Formats the calendar info as a string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Events: {:?} Holidays: {:?}", self.events, self.holidays)
    }
}

/// Draws the calendar in the given frame using the application state.
pub fn draw_calendar(app: &mut AppState, frame: &mut Frame) {
    let frame_size = frame.size();

    let constraints = vec![Constraint::Percentage(15), Constraint::Percentage(85)];

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(frame_size);

    let calendar_area = layout[1];

    let calarea = Rect {
        x: calendar_area.x,
        y: calendar_area.y,
        width: calendar_area.width,
        height: calendar_area.height,
    };

    let mut start = app.selected_date;

    let mut holidays: HashMap<Date, String> = HashMap::new();
    let holiday_info = make_dates(start.year(), app, &mut holidays);
    app.holiday_info = Some(holiday_info.clone());

    for (_i, chunk) in split_rows(&calarea)
        .iter()
        .flat_map(|row| split_cols(row).to_vec())
        .enumerate()
    {
        let cal = cals::get_cal(start.month(), start.year(), &holiday_info.events);

        // Define minimum and maximum padding values
        let min_horizontal_padding: u16 = 2;
        let max_horizontal_padding: u16 = 10;
        let min_vertical_padding: u16 = 0;
        let max_vertical_padding: u16 = 5;

        // Calculate percentage-based padding
        let horizontal_padding_percentage = 35;
        let vertical_padding_percentage = 13;

        // Calculate dynamic padding based on the frame size
        let mut horizontal_padding = (frame.size().width * horizontal_padding_percentage / 100) / 2; // Divide by 2 for padding on both sides
        let mut vertical_padding = (frame.size().height * vertical_padding_percentage / 100) / 2; // Divide by 2 for padding on both sides

        // Apply minimum and maximum constraints
        horizontal_padding = horizontal_padding
            .max(min_horizontal_padding)
            .min(max_horizontal_padding);
        vertical_padding = vertical_padding
            .max(min_vertical_padding)
            .min(max_vertical_padding);

        // Calculate the padded chunk
        let padded_chunk = Rect {
            x: chunk.x + horizontal_padding,
            y: chunk.y + vertical_padding,
            width: chunk.width.saturating_sub(horizontal_padding * 2),
            height: chunk.height.saturating_sub(vertical_padding * 2),
        };

        frame.render_widget(
            cal.block(
                Block::default()
                    .borders(Borders::all())
                    .border_type(BorderType::Rounded)
                    .title_alignment(Alignment::Center),
            ),
            padded_chunk,
        );
        if start.month().next() == Month::January {
            start = start
                .replace_day(1)
                .unwrap()
                .replace_year(start.year() + 1)
                .unwrap()
                .replace_month(start.month().next())
                .unwrap();
        } else {
            start = start
                .replace_day(1)
                .unwrap()
                .replace_month(start.month().next())
                .unwrap();
        }
    }
}

/// Splits the given area into equal horizontal rows.
fn split_rows(area: &Rect) -> Rc<[Rect]> {
    let list_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ]);

    list_layout.split(*area)
}

/// Splits the given area into equal vertical columns.
fn split_cols(area: &Rect) -> Rc<[Rect]> {
    let list_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ]);

    list_layout.split(*area)
}

/// Populates calendar info with holidays and styles for a given year.
fn make_dates(
    current_year: i32,
    app: &mut AppState,
    holidays: &mut HashMap<Date, String>,
) -> CalendarInfo {
    let mut list = match OffsetDateTime::now_local() {
        Ok(_datetime) => CalendarEventStore::today(
            Style::default()
                .add_modifier(Modifier::SLOW_BLINK)
                .fg(Color::Rgb(55, 55, 255))
                .bg(Color::Rgb(255, 255, 160)),
        ),
        Err(e) => {
            eprintln!("Failed to get local date time: {}", e);
            CalendarEventStore::today(
                Style::default()
                    .add_modifier(Modifier::SLOW_BLINK)
                    .fg(Color::Rgb(55, 55, 255))
                    .bg(Color::Rgb(255, 255, 160)),
            )
        }
    };

    let mut add_holiday = |date: Date,
                           name: &str,
                           style: Style,
                           festive_touch: &str,
                           holidays: &mut HashMap<Date, String>| {
        list.add(date, style);
        holidays.insert(date, format!("{} {}", name, festive_touch));
    };

    let holiday_style = Style::default()
        .add_modifier(Modifier::UNDERLINED)
        .fg(Color::Yellow)
        .bg(Color::Rgb(70, 100, 255));

    let selected_style = Style::default()
        .add_modifier(Modifier::CROSSED_OUT)
        .bg(Color::White);

    add_holiday(
        Date::from_calendar_date(current_year, Month::January, 1).unwrap(),
        "New Year's Day",
        holiday_style,
        "\u{1F389}", // Party Popper
        holidays,
    );

    add_holiday(
        Date::from_calendar_date(current_year, Month::February, 2).unwrap(),
        "Groundhog Day",
        holiday_style,
        "🦫", // Panda Face
        holidays,
    );

    add_holiday(
        Date::from_calendar_date(current_year, Month::February, 14).unwrap(),
        "Valentine's Day",
        holiday_style,
        "\u{1F496}", // Heart with Arrow
        holidays,
    );

    add_holiday(
        Date::from_calendar_date(current_year, Month::April, 1).unwrap(),
        "April Fool's Day",
        holiday_style,
        "🃏", // Laughing Face with Sweat
        holidays,
    );

    add_holiday(
        Date::from_calendar_date(current_year, Month::April, 22).unwrap(),
        "Earth Day",
        holiday_style,
        "🌏", //
        holidays,
    );

    add_holiday(
        Date::from_calendar_date(current_year, Month::June, 19).unwrap(),
        "Juneteenth",
        holiday_style,
        "🎆", //
        holidays,
    );

    add_holiday(
        Date::from_calendar_date(current_year, Month::July, 4).unwrap(),
        "Independence Day",
        holiday_style,
        "🕌", // United States Flag
        holidays,
    );

    add_holiday(
        Date::from_calendar_date(current_year, Month::December, 24).unwrap(),
        "Christmas Eve",
        holiday_style,
        "\u{1F384}", // Christmas Tree
        holidays,
    );

    add_holiday(
        Date::from_calendar_date(current_year, Month::December, 25).unwrap(),
        "Christmas Day",
        holiday_style,
        "\u{1F384}", // Christmas Tree
        holidays,
    );

    add_holiday(
        Date::from_calendar_date(current_year, Month::December, 31).unwrap(),
        "New Year's Eve",
        holiday_style,
        "\u{1F55B}", // Celebration
        holidays,
    );

    let season_style = Style::default()
        .fg(Color::Red)
        .bg(Color::LightYellow)
        .add_modifier(Modifier::BOLD);

    add_holiday(
        Date::from_calendar_date(current_year, Month::March, 22).unwrap(),
        "Spring Equinox",
        season_style,
        "\u{1F331}", // Seedling
        holidays,
    );

    add_holiday(
        Date::from_calendar_date(current_year, Month::June, 21).unwrap(),
        "Summer Solstice",
        season_style,
        "\u{2600}", // Sun
        holidays,
    );

    add_holiday(
        Date::from_calendar_date(current_year, Month::September, 22).unwrap(),
        "Fall Equinox",
        season_style,
        "\u{1F342}", // Maple Leaf
        holidays,
    );

    add_holiday(
        Date::from_calendar_date(current_year, Month::December, 21).unwrap(),
        "Winter Solstice",
        season_style,
        "\u{26C4}", // Snowman
        holidays,
    );

    let reference_holidays: HashMap<Date, String> = holidays.clone();

    if let Some(holiday_name) = reference_holidays.get(&app.selected_date) {
        add_holiday(
            app.selected_date,
            holiday_name,
            selected_style,
            "",
            holidays,
        );
    } else {
        add_holiday(
            app.selected_date,
            "Selected Day",
            selected_style,
            "",
            holidays,
        );
    }

    CalendarInfo {
        events: list,
        holidays: holidays.clone(),
    }
}

mod cals {
    use super::*;

    /// Fetches the calendar for a given month and year.
    pub(super) fn get_cal<'a, DS: DateStyler>(m: Month, y: i32, es: DS) -> Monthly<'a, DS> {
        default(m, y, es)
    }

    /// Creates a default style calendar for a month and year with custom styles.
    fn default<'a, DS: DateStyler>(m: Month, y: i32, es: DS) -> Monthly<'a, DS> {
        let header_style = Style::default().fg(Color::Green);
        let default_style = Style::default().fg(Color::White).bg(Color::DarkGray);

        Monthly::new(Date::from_calendar_date(y, m, 1).unwrap(), es)
            .show_surrounding(Style::default().add_modifier(Modifier::DIM))
            .show_weekdays_header(header_style)
            .default_style(default_style)
            .show_month_header(Style::default())
    }
}
