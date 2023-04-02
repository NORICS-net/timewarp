use super::TimeWarpError;
use crate::date_matcher::Direction::EndTime;
use crate::day_of_week::DayOfWeek;
use crate::doy::Doy;
use crate::error::parse_error;
use crate::month_of_year::Month;
use pest::iterators::Pairs;
use pest::Parser;
use std::str::FromStr;

#[derive(Parser, Debug, Default)]
#[grammar = "date_matcher.pest"]
struct DateMatcher;

#[derive(Eq, PartialEq, Debug)]
pub enum Direction {
    EndTime,
    StartTime,
}

fn yy_mm_dd(pairs: Pairs<'_, Rule>, today: Doy) -> Result<String, TimeWarpError> {
    let mut yy = today.year();
    let mut mm = 0;
    let mut dd = 0;
    for pair in pairs {
        match pair.as_rule() {
            Rule::yyyy => yy = i32::from_str(pair.as_str())?,
            Rule::mm => mm = i32::from_str(pair.as_str())?,
            Rule::dd => dd = i32::from_str(pair.as_str())?,
            _ => {
                println!("Found more than expected: {:?}", pair)
            }
        };
    }
    if yy < 100 {
        yy += 2000;
    }
    Ok(format!("{yy:04}-{mm:02}-{dd:02}"))
}

pub fn date_matcher(
    today: Doy,
    direction: Direction,
    date: impl Into<String>,
) -> Result<String, TimeWarpError> {
    let text = date.into();
    let mut count = 0i32;
    let mut future = direction == EndTime;
    for pair in DateMatcher::parse(Rule::date_matcher, &text)?
        .next()
        .unwrap()
        .into_inner()
    {
        match pair.as_rule() {
            Rule::date_iso | Rule::date_en | Rule::date_de => {
                return yy_mm_dd(pair.into_inner(), today)
            }
            Rule::today => return Ok(today.as_iso_date()),
            Rule::yesterday => return Ok(((today - 1) as Doy).as_iso_date()),
            Rule::tomorrow => return Ok(((today + 1) as Doy).as_iso_date()),
            Rule::last => future = false,
            Rule::next => future = true,
            Rule::forelast => {
                future = false;
                count = 1;
            }
            Rule::afternext => {
                count = 1;
            }
            Rule::day_of_week => {
                let wd_today = today.day_of_week();
                let target_wd =
                    DayOfWeek::from_day_of_week(pair.into_inner().next().unwrap().as_rule());
                let date = if future {
                    today + target_wd.days_before(wd_today) + count * 7
                } else {
                    today - wd_today.days_before(target_wd) - count * 7
                };
                return Ok(date.as_iso_date());
            }
            Rule::month => {
                let month = Month::from_month(pair.into_inner().next().unwrap().as_rule());
                let date = find_rel_month(today, direction, future, month);
                return Ok(date.as_iso_date());
            }

            _ => println!("date_matcher :: {:?}", pair),
        };
    }
    parse_error("Nothing found")
}

fn find_rel_month(today: Doy, direction: Direction, future: bool, target_month: Month) -> Doy {
    // if direction is EndTime add a Month
    let target_month = target_month.inc((direction == EndTime) as i32);
    let today_m = today.month();
    let add = if target_month > today_m && !future {
        -1
    } else if target_month < today_m && future {
        1
    } else {
        0
    };
    let date = Doy::from_ymd(today.year() + add, target_month as i32, 1);
    println!("{target_month:?} / {today_m:?} == {date}");
    date
}

#[cfg(test)]
mod should {
    use super::date_matcher;
    use crate::date_matcher::find_rel_month;
    use crate::date_matcher::Direction::{EndTime, StartTime};
    use crate::doy::Doy;
    use crate::month_of_year::Month::{Aug, Jan};

    #[test]
    fn find_relative_months() {
        // Fri 2023-03-17
        let today = Doy::from_ymd(2023, 3, 17);
        assert_eq!(
            Doy::new(1, 2023),
            find_rel_month(today, StartTime, false, Jan)
        );

        assert_eq!(
            date_matcher(today, StartTime, "last january").unwrap(),
            "2023-01-01"
        );
        assert_eq!(
            date_matcher(today, StartTime, "next january").unwrap(),
            "2024-01-01"
        );
        assert_eq!(
            Doy::from_ymd(2023, 9, 1),
            find_rel_month(today, EndTime, true, Aug)
        );
        assert_eq!(
            Doy::from_ymd(2022, 8, 1),
            find_rel_month(today, StartTime, false, Aug)
        );
        assert_eq!(
            date_matcher(today, EndTime, "next january").unwrap(),
            "2024-02-01"
        );
    }

    #[test]
    fn find_relative_dates() {
        // Fri 2023-03-17
        let today = Doy::from_ymd(2023, 3, 17);
        assert_eq!(
            date_matcher(today, EndTime, "last monday").unwrap(),
            "2023-03-13"
        );
        assert_eq!(
            date_matcher(today, StartTime, "tuesday").unwrap(),
            "2023-03-14"
        );
        assert_eq!(
            date_matcher(today, EndTime, "tuesday").unwrap(),
            "2023-03-21"
        );
        assert_eq!(
            date_matcher(today, StartTime, "letzten donnerstag").unwrap(),
            "2023-03-16"
        );
        assert_eq!(
            date_matcher(today, EndTime, "last friday").unwrap(),
            "2023-03-10"
        );
        assert_eq!(
            date_matcher(today, EndTime, "nächsten Fr").unwrap(),
            "2023-03-24"
        );
        assert_eq!(
            date_matcher(today, EndTime, "coming Thu").unwrap(),
            "2023-03-23"
        );
        assert_eq!(
            date_matcher(today, EndTime, "übernächsten Donnerstag").unwrap(),
            "2023-03-30"
        );
        assert_eq!(
            date_matcher(today, EndTime, "nächster Mo").unwrap(),
            "2023-03-20"
        );
        assert_eq!(
            date_matcher(today, EndTime, "vorletzter mo").unwrap(),
            "2023-03-06"
        );
    }

    #[test]
    fn find_yesterday() {
        let first_of_march = Doy::from_ymd(2023, 3, 1);
        assert_eq!(
            date_matcher(first_of_march, EndTime, "heute").unwrap(),
            "2023-03-01"
        );
        assert_eq!(
            date_matcher(first_of_march, EndTime, "yesterday").unwrap(),
            "2023-02-28"
        );
        assert_eq!(
            date_matcher(first_of_march, EndTime, "morgen").unwrap(),
            "2023-03-02"
        );
    }

    #[test]
    fn parse_date() {
        // Fri 2023-03-17
        let today = Doy::from_ymd(2023, 3, 17);
        assert_eq!(
            date_matcher(today, StartTime, "22.01.2023").unwrap(),
            "2023-01-22"
        );
        assert_eq!(
            date_matcher(today, StartTime, "22.1.23").unwrap(),
            "2023-01-22"
        );
        assert_eq!(
            date_matcher(today, StartTime, "22.1.").unwrap(),
            "2023-01-22"
        );
        assert_eq!(
            date_matcher(today, StartTime, "3/16/2023").unwrap(),
            "2023-03-16"
        );
        assert_eq!(
            date_matcher(today, StartTime, "2023-03-16").unwrap(),
            "2023-03-16"
        );
        assert_eq!(
            date_matcher(today, StartTime, "    23-03-16  ").unwrap(),
            "2023-03-16"
        );
    }
}
