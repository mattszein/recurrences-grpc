use super::DataRrule;
use chrono::Month;
use chrono::TimeZone;
use rrule::{NWeekday, RRule, RRuleResult, RRuleSet, Tz, Weekday};

/*
pub struct DataRrule {
    by_week_day: Vec<String>,
    by_hour: Vec<u32>,
    by_minute: Vec<u32>,
    by_month: Vec<u32>,
    by_month_day: Vec<i32>,
    by_second: Vec<u32>,
    by_set_pos: Vec<i32>,
    by_week_no: Vec<i32>,
    by_year_day: Vec<i32>,
    count: u32,
    dt_end: String,
    dt_start: String,
    freq: String,
    interval: u32,
    until: String,
    wkst: i32,
}
*/
pub fn rrule_from_string(rrule: &String) -> RRuleResult {
    let rrule_set: RRuleSet = rrule.parse().unwrap();
    rrule_set.all(100)
}

pub fn rrule_from_data(request: &DataRrule) -> RRuleResult {
    let rrule_set = RRule::default()
        .count(request.count)
        .freq(request.freq.parse().unwrap())
        .week_start(Weekday::try_from(u8::try_from(request.week_start).unwrap()).unwrap())
        .by_year_day(
            request
                .by_year_day
                .iter()
                .map(|&x| i16::try_from(x).unwrap())
                .collect(),
        )
        .by_month(
            &request
                .by_month
                .iter()
                .map(|&x| Month::try_from(u8::try_from(x).unwrap()).unwrap())
                .collect::<Vec<chrono::Month>>(),
        )
        .by_month_day(
            request
                .by_month_day
                .iter()
                .map(|&x| i8::try_from(x).unwrap())
                .collect(),
        )
        .by_weekday(
            request
                .by_week_day
                .iter()
                .map(|x| x.parse::<NWeekday>().unwrap())
                .collect(),
        )
        .by_week_no(
            request
                .by_week_no
                .iter()
                .map(|&x| i8::try_from(x).unwrap())
                .collect(),
        )
        .by_hour(
            request
                .by_hour
                .iter()
                .map(|&x| u8::try_from(x).unwrap())
                .collect(),
        )
        .by_minute(
            request
                .by_minute
                .iter()
                .map(|&x| u8::try_from(x).unwrap())
                .collect(),
        )
        .by_second(
            request
                .by_second
                .iter()
                .map(|&x| u8::try_from(x).unwrap())
                .collect(),
        )
        .by_set_pos(request.by_set_pos.clone())
        .interval(if request.interval == 0 {
            1
        } else {
            u16::try_from(request.interval).unwrap()
        })
        .build(Tz::UTC.with_ymd_and_hms(2023, 8, 1, 0, 0, 0).unwrap())
        .expect("RRule invalid");
    rrule_set.all(100)
}
