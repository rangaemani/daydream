use crate::app::AppState;
use ratatui::{prelude::*, widgets::calendar::*};
use std::rc::Rc;
use time::{Date, Month};

//TODO - for selected date, show any journal entries tagged for that date. for journal thumbnail show date, entry title, and tags

pub struct DatePosition {
    date: Date,
    position: (i16, i16),
}

pub fn draw_calendar(app: &mut AppState, frame: &mut Frame) {
    let app_area = frame.size();

    let calarea = Rect {
        x: app_area.x
            + if app_area.height / 9 == 0 {
                1
            } else {
                app_area.height / 9
            },
        y: app_area.y
            + if app_area.height / 7 == 0 {
                1
            } else {
                app_area.height / 7
            },
        height: if app_area.height > 1 {
            app_area.height - 1
        } else {
            1
        },
        width: if app_area.width > 1 {
            app_area.width - 1
        } else {
            1
        },
    };

    let mut start = app.selected_date;

    let list = make_dates(start.year(), app);

    let mut date_pos_map: Vec<DatePosition> = Vec::new();

    for (i, chunk) in split_rows(&calarea)
        .iter()
        .flat_map(|row| split_cols(row).to_vec())
        .enumerate()
    {
        let cal = cals::get_cal(start.month(), start.year(), &list);
        let pos = (
            (i % calarea.x as usize) as i16,
            (i / calarea.y as usize) as i16,
        ); // Calculate the position
        date_pos_map.push(DatePosition {
            date: start,
            position: pos,
        }); // Store the date-position mapping
        frame.render_widget(cal, chunk);
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
    app.date_pos_map = date_pos_map;
}

pub fn map_to_date(app: &AppState, x: i16, y: i16) -> Option<Date> {
    app.date_pos_map.iter().find_map(|date_pos| {
        if date_pos.position.0 - x < 3 && date_pos.position.1 - y < 3 {
            Some(date_pos.date)
        } else {
            None
        }
    })
}

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

fn make_dates(current_year: i32, app: &mut AppState) -> CalendarEventStore {
    // let cal = bdays::calendars::us::USSettlement;

    let mut list = CalendarEventStore::today(
        Style::default()
            .add_modifier(Modifier::SLOW_BLINK)
            .fg(Color::Rgb(55, 55, 255))
            .bg(Color::Rgb(255, 255, 160)),
    );

    // Weekend
    let _weekend_style = Style::default()
        .add_modifier(Modifier::ITALIC)
        .bg(Color::LightBlue);

    // Holidays
    let holiday_style = Style::default()
        .add_modifier(Modifier::UNDERLINED)
        .fg(Color::Yellow)
        .bg(Color::Rgb(70, 100, 255));

    // selected style
    let selected_style = Style::default()
        .add_modifier(Modifier::CROSSED_OUT)
        .bg(Color::White);
    // new year's
    list.add(
        Date::from_calendar_date(current_year, Month::January, 1).unwrap(),
        holiday_style,
    );
    // next new_year's day
    list.add(
        Date::from_calendar_date(current_year + 1, Month::January, 1).unwrap(),
        holiday_style,
    );
    // groundhog day
    list.add(
        Date::from_calendar_date(current_year, Month::February, 2).unwrap(),
        holiday_style,
    );
    // valentine's day
    list.add(
        Date::from_calendar_date(current_year, Month::February, 14).unwrap(),
        holiday_style,
    );
    // april fool's
    list.add(
        Date::from_calendar_date(current_year, Month::April, 1).unwrap(),
        holiday_style,
    );
    // earth day
    list.add(
        Date::from_calendar_date(current_year, Month::April, 22).unwrap(),
        holiday_style,
    );
    // juneteenth
    list.add(
        Date::from_calendar_date(current_year, Month::June, 19).unwrap(),
        holiday_style,
    );
    // independence day
    list.add(
        Date::from_calendar_date(current_year, Month::July, 4).unwrap(),
        holiday_style,
    );
    // christmas eve
    list.add(
        Date::from_calendar_date(current_year, Month::December, 24).unwrap(),
        holiday_style,
    );
    // christmas day
    list.add(
        Date::from_calendar_date(current_year, Month::December, 25).unwrap(),
        holiday_style,
    );
    // new year's eve
    list.add(
        Date::from_calendar_date(current_year, Month::December, 31).unwrap(),
        holiday_style,
    );

    // seasons
    let season_style = Style::default()
        .fg(Color::Red)
        .bg(Color::LightYellow)
        .add_modifier(Modifier::BOLD);
    // spring equinox
    list.add(
        Date::from_calendar_date(current_year, Month::March, 22).unwrap(),
        season_style,
    );
    // summer solstice
    list.add(
        Date::from_calendar_date(current_year, Month::June, 21).unwrap(),
        season_style,
    );
    // fall equinox
    list.add(
        Date::from_calendar_date(current_year, Month::September, 22).unwrap(),
        season_style,
    );
    list.add(
        Date::from_calendar_date(current_year, Month::December, 21).unwrap(),
        season_style,
    );
    // currently selected day
    list.add(app.selected_date, selected_style);

    // // Use the custom holiday calendar to generate additional holidays
    // for month in 1..=12 {
    //     for day in 1..=31 {
    //         let date = NaiveDate::from_ymd_opt(current_year, month, day).unwrap();
    //         let days_since_ce = date.num_days_from_ce();
    //         let year = ((days_since_ce + 365) / 365) + 1970;
    //         let month = ((days_since_ce % 365) / 30) + 1;
    //         let day = (days_since_ce % 30) + 1;
    //         let candidate_date = Date::from_calendar_date(
    //             OffsetDateTime::UNIX_EPOCH.to_calendar_date().0.add(year),
    //             OffsetDateTime::UNIX_EPOCH
    //                 .to_calendar_date()
    //                 .1
    //                 .nth_next(month as u8),
    //             OffsetDateTime::UNIX_EPOCH
    //                 .to_calendar_date()
    //                 .2
    //                 .add(day as u8),
    //         );
    //         if bdays::is_weekend(date) {
    //             list.add(dbg!(candidate_date.unwrap()), weekend_style);
    //         } else if cal.is_holiday(date) {
    //             list.add(candidate_date.unwrap(), holiday_style);
    //         }
    //     }
    // }

    list
}

mod cals {
    use super::*;

    pub(super) fn get_cal<'a, DS: DateStyler>(m: Month, y: i32, es: DS) -> Monthly<'a, DS> {
        // use Month::*;
        match m {
            // May => example3(m, y, es),
            // June => example3(m, y, es),
            // July => example3(m, y, es),
            // December => example3(m, y, es),
            // February => example3(m, y, es),
            // November => example3(m, y, es),
            _ => default(m, y, es),
        }
    }

    fn default<'a, DS: DateStyler>(m: Month, y: i32, es: DS) -> Monthly<'a, DS> {
        let header_style = Style::default().fg(Color::Green);

        let default_style = Style::default().fg(Color::White).bg(Color::LightBlue);

        Monthly::new(Date::from_calendar_date(y, m, 1).unwrap(), es)
            .show_surrounding(Style::default().add_modifier(Modifier::DIM))
            .show_weekdays_header(header_style)
            .default_style(default_style)
            .show_month_header(Style::default())
    }

    fn _style1<'a, DS: DateStyler>(m: Month, y: i32, es: DS) -> Monthly<'a, DS> {
        let default_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Rgb(50, 50, 50));

        Monthly::new(Date::from_calendar_date(y, m, 1).unwrap(), es)
            .show_surrounding(default_style)
            .default_style(default_style)
            .show_month_header(Style::default())
    }

    fn _style2<'a, DS: DateStyler>(m: Month, y: i32, es: DS) -> Monthly<'a, DS> {
        let header_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::DIM)
            .fg(Color::LightYellow);

        let default_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Rgb(50, 50, 50));

        Monthly::new(Date::from_calendar_date(y, m, 1).unwrap(), es)
            .show_weekdays_header(header_style)
            .default_style(default_style)
            .show_month_header(Style::default())
    }

    fn _style3<'a, DS: DateStyler>(m: Month, y: i32, es: DS) -> Monthly<'a, DS> {
        let default_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Rgb(50, 50, 50));

        Monthly::new(Date::from_calendar_date(y, m, 1).unwrap(), es)
            .show_month_header(Style::default())
            .default_style(default_style)
    }

    fn _style4<'a, DS: DateStyler>(m: Month, y: i32, es: DS) -> Monthly<'a, DS> {
        let header_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Green);

        let default_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Rgb(50, 50, 50));

        Monthly::new(Date::from_calendar_date(y, m, 1).unwrap(), es)
            .show_weekdays_header(header_style)
            .default_style(default_style)
    }

    fn _style5<'a, DS: DateStyler>(m: Month, y: i32, es: DS) -> Monthly<'a, DS> {
        let header_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Green);

        let default_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Rgb(50, 50, 50));

        Monthly::new(Date::from_calendar_date(y, m, 1).unwrap(), es)
            .show_month_header(header_style)
            .default_style(default_style)
    }
}
