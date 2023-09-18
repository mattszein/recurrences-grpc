mod parser;
use super::DataRrule;
use chrono::Month;
use chrono::TimeZone;
use rrule::{NWeekday, RRule, RRuleResult, RRuleSet, Tz, Weekday};
use std::error::Error;

pub struct ProcessResult {
    pub rrule_result: Option<RRuleResult>,
    pub rrule: String,
    pub valid: bool,
    pub errors: Vec<String>,
}

pub fn rrule_from_string(rrule: &String) -> Result<ProcessResult, Box<dyn Error>> {
    let rrule_set: RRuleSet = match rrule.parse() {
        Ok(x) => x,
        Err(e) => Err(e)?,
    };
    let rrule_string = rrule_set.to_string();
    let dates = rrule_set.all(100);
    Ok(ProcessResult {
        rrule_result: Some(dates),
        rrule: rrule_string,
        valid: true,
        errors: vec![],
    })
}

pub fn rrule_from_data(request: &DataRrule) -> Result<ProcessResult, Box<dyn Error>> {
    let dt_start_dt = parser::process_datetime_field(&request.dt_start, "dt_start")?;
    let by_month_days: Vec<i8> = parser::process_field(
        &request.by_month_day,
        |x| i8::try_from(*x).map_err(|e| e.to_string()),
        "by_month_day",
    )?;

    let by_year_days: Vec<i16> = parser::process_field(
        &request.by_year_day,
        |x| i16::try_from(*x).map_err(|e| e.to_string()),
        "by_year_day",
    )?;

    let by_months: Vec<Month> = parser::process_field(
        &request.by_month,
        |x| {
            Month::try_from(u8::try_from(*x).map_err(|e| e.to_string())?).map_err(|e| e.to_string())
        },
        "by_month",
    )?;

    let by_week_days: Vec<NWeekday> = parser::process_field(
        &request.by_week_day,
        |x| x.parse::<NWeekday>().map_err(|e| e.to_string()),
        "by_week_day",
    )?;

    let by_week_nos: Vec<i8> = parser::process_field(
        &request.by_week_no,
        |x| i8::try_from(*x).map_err(|e| e.to_string()),
        "by_week_no",
    )?;

    let by_hours: Vec<u8> = parser::process_field(
        &request.by_hour,
        |x| u8::try_from(*x).map_err(|e| e.to_string()),
        "by_hour",
    )?;

    let by_minutes: Vec<u8> = parser::process_field(
        &request.by_minute,
        |x| u8::try_from(*x).map_err(|e| e.to_string()),
        "by_minute",
    )?;

    let by_seconds: Vec<u8> = parser::process_field(
        &request.by_second,
        |x| u8::try_from(*x).map_err(|e| e.to_string()),
        "by_second",
    )?;

    let mut rrule = RRule::default()
        .freq(request.freq.parse().unwrap())
        .week_start(Weekday::try_from(u8::try_from(request.week_start).unwrap()).unwrap())
        .by_year_day(by_year_days)
        .by_month(&by_months)
        .by_month_day(by_month_days)
        .by_weekday(by_week_days)
        .by_week_no(by_week_nos)
        .by_hour(by_hours)
        .by_minute(by_minutes)
        .by_second(by_seconds)
        .by_set_pos(request.by_set_pos.clone())
        .interval(if request.interval == 0 {
            1
        } else {
            u16::try_from(request.interval).unwrap()
        });
    if !request.until.is_empty() {
        rrule = rrule.until(
            Tz::UTC.from_utc_datetime(&parser::process_datetime_field(&request.until, "until")?),
        );
    }
    if request.count != 0 {
        rrule = rrule.count(request.count)
    }
    let rrule_set = match rrule.build(Tz::UTC.from_utc_datetime(&dt_start_dt)) {
        Ok(x) => x,
        Err(e) => Err(e)?,
    };
    let rrule_string = rrule_set.to_string();
    let dates = rrule_set.all(100);
    Ok(ProcessResult {
        rrule_result: Some(dates),
        rrule: rrule_string,
        valid: true,
        errors: vec![],
    })
}
