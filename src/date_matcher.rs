use super::TimeWarpError;
use crate::date_matcher::Direction::EndTime;
use crate::day_of_week::DayOfWeek;
use crate::doy::{DaySpan, Doy};
use crate::error::parse_error;
use crate::month_of_year::Month;
use pest::iterators::Pairs;
use pest::Parser;
use std::str::FromStr;

#[derive(Parser, Debug, Default)]
#[grammar = "date_matcher.pest"]
struct DateMatcher;

/// Designated use of the date.
#[derive(Eq, PartialEq, Debug)]
pub enum Direction {
    EndTime,
    StartTime,
}

fn ok_doy(d: Doy) -> Result<DaySpan, TimeWarpError> {
    Ok(DaySpan::Doy(d))
}

fn yy_mm_dd(pairs: Pairs<'_, Rule>, today: Doy) -> Result<DaySpan, TimeWarpError> {
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
    ok_doy(Doy::from_ymd(yy, mm, dd))
}

pub fn date_matcher(
    today: Doy,
    direction: Direction,
    date: impl Into<String>,
) -> Result<DaySpan, TimeWarpError> {
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
            Rule::today => return ok_doy(today),
            Rule::yesterday => return ok_doy(today - 1),
            Rule::tomorrow => return ok_doy(today + 1),
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
                return ok_doy(date);
            }
            Rule::month => {
                let month = Month::from_month(pair.into_inner().next().unwrap().as_rule());
                let date = find_rel_month(today, direction, future, month);
                return ok_doy(date);
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
    Doy::from_ymd(today.year() + add, target_month as i32, 1)
}

#[cfg(test)]
mod should {
    use super::date_matcher;
    use crate::date_matcher::find_rel_month;
    use crate::date_matcher::Direction::{EndTime, StartTime};
    use crate::doy::{DaySpan, Doy};
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
            DaySpan::Doy(Doy::new(1, 2023)),
            date_matcher(today, StartTime, "last january").unwrap(),
        );
        assert_eq!(
            DaySpan::Doy(Doy::new(1, 2024)),
            date_matcher(today, StartTime, "next january").unwrap(),
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
            DaySpan::Doy(Doy::from_ymd(2024, 2, 1)),
            date_matcher(today, EndTime, "next january").unwrap(),
        );
    }

    #[test]
    fn find_relative_dates() {
        // Fri 2023-03-17
        let today = Doy::from_ymd(2023, 3, 17);
        assert_eq!(
            DaySpan::Doy(Doy::from_ymd(2023, 3, 13)),
            date_matcher(today, EndTime, "last monday").unwrap(),
        );
        assert_eq!(
            DaySpan::Doy(Doy::from_ymd(2023, 3, 14)),
            date_matcher(today, StartTime, "tuesday").unwrap(),
        );
        assert_eq!(
            DaySpan::Doy(Doy::from_ymd(2023, 3, 21)),
            date_matcher(today, EndTime, "tuesday").unwrap(),
        );
        assert_eq!(
            DaySpan::Doy(Doy::from_ymd(2023, 3, 16)),
            date_matcher(today, StartTime, "letzten donnerstag").unwrap(),
        );
        assert_eq!(
            DaySpan::Doy(Doy::from_ymd(2023, 3, 10)),
            date_matcher(today, EndTime, "last friday").unwrap(),
        );
        assert_eq!(
            DaySpan::Doy(Doy::from_ymd(2023, 3, 24)),
            date_matcher(today, EndTime, "nächsten Fr").unwrap(),
        );
        assert_eq!(
            DaySpan::Doy(Doy::from_ymd(2023, 3, 23)),
            date_matcher(today, EndTime, "coming Thu").unwrap(),
        );
        assert_eq!(
            DaySpan::Doy(Doy::from_ymd(2023, 3, 30)),
            date_matcher(today, EndTime, "übernächsten Donnerstag").unwrap(),
        );
        assert_eq!(
            DaySpan::Doy(Doy::from_ymd(2023, 3, 20)),
            date_matcher(today, EndTime, "nächster Mo").unwrap(),
        );
        assert_eq!(
            DaySpan::Doy(Doy::from_ymd(2023, 3, 6)),
            date_matcher(today, EndTime, "vorletzter mo").unwrap(),
        );
    }

    #[test]
    fn find_yesterday() {
        let first_of_march = Doy::from_ymd(2023, 3, 1);
        assert_eq!(
            DaySpan::Doy(Doy::from_ymd(2023, 3, 1)),
            date_matcher(first_of_march, EndTime, "heute").unwrap(),
        );
        assert_eq!(
            DaySpan::Doy(Doy::from_ymd(2023, 2, 28)),
            date_matcher(first_of_march, EndTime, "yesterday").unwrap(),
        );
        assert_eq!(
            DaySpan::Doy(Doy::from_ymd(2023, 3, 2)),
            date_matcher(first_of_march, EndTime, "morgen").unwrap(),
        );
    }

    #[test]
    fn parse_date() {
        // Fri 2023-03-17
        let today = Doy::from_ymd(2023, 3, 17);
        assert_eq!(
            DaySpan::Doy(Doy::from_ymd(2023, 1, 22)),
            date_matcher(today, StartTime, "22.01.2023").unwrap(),
        );
        assert_eq!(
            DaySpan::Doy(Doy::from_ymd(2023, 1, 22)),
            date_matcher(today, StartTime, "22.1.23").unwrap(),
        );
        assert_eq!(
            DaySpan::Doy(Doy::from_ymd(2023, 1, 22)),
            date_matcher(today, StartTime, "22.1.").unwrap(),
        );
        assert_eq!(
            DaySpan::Doy(Doy::from_ymd(2023, 3, 16)),
            date_matcher(today, StartTime, "3/16/2023").unwrap(),
        );
        assert_eq!(
            DaySpan::Doy(Doy::from_ymd(2023, 3, 16)),
            date_matcher(today, StartTime, "2023-03-16").unwrap(),
        );
        assert_eq!(
            DaySpan::Doy(Doy::from_ymd(2023, 3, 16)),
            date_matcher(today, StartTime, "    23-03-16  ").unwrap(),
        );
    }
}
