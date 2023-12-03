use super::TimeWarpError;
use crate::day_of_week::DayOfWeek;
use crate::doy::{Doy, Tempus};
use crate::error::parse_error;
use crate::month_of_year::Month;
use pest::iterators::Pairs;
use pest::Parser;
use std::str::FromStr;

#[derive(Parser, Debug, Default)]
#[grammar = "date_matcher.pest"]
struct DateMatcher;

/// Designated use of the date.
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    To,
    From,
}

fn ok_doy(d: Doy) -> Result<Tempus, TimeWarpError> {
    Ok(Tempus::Moment(d))
}

fn correct_yyyy(str: &str, relative: i32) -> Result<i32, TimeWarpError> {
    let yy = i32::from_str(str)?;
    if yy > 100 {
        return Ok(yy);
    }
    let offset = relative % 100;
    let base = relative - offset;
    if yy > offset + 50 {
        Ok(base - 100 + yy)
    } else if yy < offset - 50 {
        Ok(base + 100 + yy)
    } else {
        Ok(base + yy)
    }
}

fn yy_mm_dd(pairs: Pairs<'_, Rule>, today: Doy) -> Result<Tempus, TimeWarpError> {
    let mut yy = today.year;
    let mut mm = 0;
    let mut dd = 0;
    for pair in pairs {
        match pair.as_rule() {
            Rule::yyyy => yy = correct_yyyy(pair.as_str(), today.year)?,
            Rule::mm => mm = i32::from_str(pair.as_str())?,
            Rule::dd => dd = i32::from_str(pair.as_str())?,
            _ => println!("Found more than expected: {pair:?}"),
        };
    }
    ok_doy(Doy::from_ymd(yy, mm, dd))
}

fn date_lang(pairs: Pairs<'_, Rule>, today: Doy) -> Result<Tempus, TimeWarpError> {
    let mut yy = today.year;
    let mut mm = 0;
    let mut dd = 0;
    for pair in pairs {
        match pair.as_rule() {
            Rule::yyyy => yy = correct_yyyy(pair.as_str(), today.year)?,
            Rule::month => {
                mm = Month::from_month(pair.into_inner().next().unwrap().as_rule()) as i32;
            }
            Rule::dd => dd = i32::from_str(pair.as_str())?,
            _ => println!("Found more than expected: {pair:?}"),
        };
    }
    if yy < 100 {
        yy += 2000;
    }
    ok_doy(Doy::from_ymd(yy, mm, dd))
}

fn date_week(pairs: Pairs<'_, Rule>, today: Doy) -> Result<Tempus, TimeWarpError> {
    let mut yy = today.year;
    let mut kw = 0;
    for pair in pairs {
        match pair.as_rule() {
            Rule::yyyy => yy = correct_yyyy(pair.as_str(), today.year)?,
            Rule::kw => kw = i32::from_str(pair.as_str())?,
            _ => println!("Found more than expected: {pair:?}"),
        }
    }
    let start = Doy::from_week(yy, kw);
    Ok(Tempus::Interval(start, start + 7))
}

pub fn date_matcher(
    today: Doy,
    direction: Direction,
    date: impl Into<String>,
) -> Result<Tempus, TimeWarpError> {
    let text = date.into();
    let mut amount = 0i32;
    let mut future = direction == Direction::To;
    for pair in DateMatcher::parse(Rule::date_matcher, &text)?
        .next()
        .unwrap()
        .into_inner()
    {
        match pair.as_rule() {
            Rule::date_iso | Rule::date_en | Rule::date_de => {
                return yy_mm_dd(pair.into_inner(), today)
            }
            Rule::date_lang => return date_lang(pair.into_inner(), today),
            Rule::date_kw => return date_week(pair.into_inner(), today),
            Rule::today => return ok_doy(today),
            Rule::yesterday => return ok_doy(today - 1),
            Rule::tomorrow => return ok_doy(today + 1),
            Rule::last => future = false,
            Rule::next => future = true,
            Rule::amount => amount = i32::from_str(pair.as_str())?,
            Rule::forelast => {
                future = false;
                amount = 1;
            }
            Rule::afternext => {
                amount = 1;
            }
            Rule::day_of_week => {
                let wd_today = today.day_of_week();
                let target_wd =
                    DayOfWeek::from_day_of_week(pair.into_inner().next().unwrap().as_rule());
                let date = if future {
                    today + target_wd.days_before(wd_today) + amount * 7
                } else {
                    today - wd_today.days_before(target_wd) - amount * 7
                };
                return ok_doy(date);
            }
            Rule::month => {
                let month = Month::from_month(pair.into_inner().next().unwrap().as_rule());
                let date = find_rel_month(today, direction, future, month);
                return ok_doy(date);
            }
            Rule::timeunit => {
                return ok_doy(find_timeunit(
                    pair.into_inner().next().unwrap().as_rule(),
                    today,
                    amount,
                ))
            }
            _ => println!("date_matcher :: {pair:?}"),
        };
    }
    parse_error("Nothing found")
}

fn find_rel_month(today: Doy, direction: Direction, future: bool, target_month: Month) -> Doy {
    // if direction is EndTime add a Month
    let target_month = target_month.inc(i32::from(direction == Direction::To));
    let today_m = today.month();
    let add = if target_month > today_m && !future {
        -1
    } else {
        i32::from(target_month < today_m && future)
    };
    Doy::from_ymd(today.year + add, target_month as i32, 1)
}

fn find_timeunit(rule: Rule, today: Doy, amount: i32) -> Doy {
    match rule {
        Rule::days => today + amount,
        Rule::months => {
            let mut m = today.month() as i32 + amount;
            let mut y = today.year;
            while m > 12 {
                m -= 12;
                y += 1;
            }
            while m < 1 {
                m += 12;
                y -= 1;
            }
            Doy::from_ymd(y, m, today.day_of_month())
        }
        Rule::years => Doy::new(today.doy, today.year + amount),
        _ => {
            println!("find_timeunit :: {rule:?}");
            today
        }
    }
}

#[cfg(test)]
mod should {
    use crate::date_matcher;
    use crate::date_matcher::{correct_yyyy, find_rel_month};
    use crate::Direction::{From, To};
    use crate::Month::{Aug, Jan};
    use crate::{Doy, Tempus};

    #[test]
    fn adjust_yyyy() {
        assert_eq!(2023, correct_yyyy("2023", 2023).unwrap());
        assert_eq!(2023, correct_yyyy("23", 2023).unwrap());
        assert_eq!(2023, correct_yyyy("23", 1995).unwrap());
        assert_eq!(1989, correct_yyyy("89", 2023).unwrap());
        assert_eq!(2089, correct_yyyy("89", 2043).unwrap());
    }

    #[test]
    fn find_relative_months() {
        // Fri 2023-03-17
        let today = Doy::from_ymd(2023, 3, 17);
        assert_eq!(Doy::new(1, 2023), find_rel_month(today, From, false, Jan));

        assert_eq!(
            Tempus::Moment(Doy::new(1, 2023)),
            date_matcher(today, From, "last january").unwrap(),
        );
        assert_eq!(
            Tempus::Moment(Doy::new(1, 2024)),
            date_matcher(today, From, "next january").unwrap(),
        );
        assert_eq!(
            Doy::from_ymd(2023, 9, 1),
            find_rel_month(today, To, true, Aug)
        );
        assert_eq!(
            Doy::from_ymd(2022, 8, 1),
            find_rel_month(today, From, false, Aug)
        );
        assert_eq!(
            Doy::from_ymd(2022, 9, 1),
            find_rel_month(today, To, false, Aug)
        );
        assert_eq!(
            Tempus::Moment(Doy::from_ymd(2024, 2, 1)),
            date_matcher(today, To, "next january").unwrap(),
        );
    }

    #[test]
    fn find_relative_dates() {
        // Fri 2023-03-17
        let today = Doy::from_ymd(2023, 3, 17);
        assert_eq!(
            Tempus::Moment(Doy::from_ymd(2023, 3, 13)),
            date_matcher(today, To, "last monday").unwrap(),
        );
        assert_eq!(
            Tempus::Moment(Doy::from_ymd(2023, 3, 14)),
            date_matcher(today, From, "tuesday").unwrap(),
        );
        assert_eq!(
            Tempus::Moment(Doy::from_ymd(2023, 3, 21)),
            date_matcher(today, To, "tuesday").unwrap(),
        );
        assert_eq!(
            Tempus::Moment(Doy::from_ymd(2023, 3, 16)),
            date_matcher(today, From, "letzten donnerstag").unwrap(),
        );
        assert_eq!(
            Tempus::Moment(Doy::from_ymd(2023, 3, 10)),
            date_matcher(today, To, "last friday").unwrap(),
        );
        assert_eq!(
            Tempus::Moment(Doy::from_ymd(2023, 3, 24)),
            date_matcher(today, To, "nächsten Fr").unwrap(),
        );
        assert_eq!(
            Tempus::Moment(Doy::from_ymd(2023, 3, 23)),
            date_matcher(today, To, "coming Thu").unwrap(),
        );
        assert_eq!(
            Tempus::Moment(Doy::from_ymd(2023, 3, 30)),
            date_matcher(today, To, "übernächsten Donnerstag").unwrap(),
        );
        assert_eq!(
            Tempus::Moment(Doy::from_ymd(2023, 3, 20)),
            date_matcher(today, To, "nächster Mo").unwrap(),
        );
        assert_eq!(
            Tempus::Moment(Doy::from_ymd(2023, 3, 6)),
            date_matcher(today, To, "vorletzter mo").unwrap(),
        );
    }

    #[test]
    fn find_yesterday() {
        let first_of_march = Doy::from_ymd(2023, 3, 1);
        assert_eq!(
            Tempus::Moment(Doy::from_ymd(2023, 3, 1)),
            date_matcher(first_of_march, To, "heute").unwrap(),
        );
        assert_eq!(
            Tempus::Moment(Doy::from_ymd(2023, 2, 28)),
            date_matcher(first_of_march, To, "yesterday").unwrap(),
        );
        assert_eq!(
            Tempus::Moment(Doy::from_ymd(2023, 3, 2)),
            date_matcher(first_of_march, To, "morgen").unwrap(),
        );
    }

    #[test]
    fn adding_times() {
        // Fri 2023-03-17
        let today = Doy::from_ymd(2023, 3, 17);
        assert_eq!(
            Tempus::Moment(Doy::from_ymd(2023, 3, 22)),
            date_matcher(today, From, "+5 Tage").unwrap(),
        );
        let today = Doy::from_ymd(2023, 3, 17);
        assert_eq!(
            Tempus::Moment(Doy::from_ymd(2022, 3, 17)),
            date_matcher(today, From, "-1 year").unwrap(),
        );
        assert_eq!(
            Tempus::Moment(Doy::from_ymd(2022, 2, 17)),
            date_matcher(today, From, "-13 month").unwrap(),
        );
    }

    #[test]
    fn parse_date() {
        // Fri 2023-03-17
        let today = Doy::from_ymd(2023, 3, 17);
        assert_eq!(
            Tempus::Moment(Doy::from_ymd(2023, 1, 22)),
            date_matcher(today, From, "22.01.2023").unwrap(),
        );
        assert_eq!(
            Tempus::Moment(Doy::from_ymd(2023, 1, 22)),
            date_matcher(today, From, "22.1.23").unwrap(),
        );
        assert_eq!(
            Tempus::Moment(Doy::from_ymd(2023, 1, 22)),
            date_matcher(today, From, "22.1.").unwrap(),
        );
        assert_eq!(
            Tempus::Moment(Doy::from_ymd(2023, 3, 16)),
            date_matcher(today, From, "3/16/2023").unwrap(),
        );
        assert_eq!(
            Tempus::Moment(Doy::from_ymd(2023, 3, 16)),
            date_matcher(today, From, "2023-03-16").unwrap(),
        );
        assert_eq!(
            Tempus::Moment(Doy::from_ymd(2023, 3, 16)),
            date_matcher(today, From, "    23-03-16  ").unwrap(),
        );

        assert_eq!(
            Tempus::Moment(Doy::from_ymd(2023, 3, 16)),
            date_matcher(today, From, "16. Mär 2023").unwrap(),
        );
        assert_eq!(
            Tempus::Moment(Doy::from_ymd(2023, 3, 16)),
            date_matcher(today, From, "16. März 2023").unwrap(),
        );
        assert_eq!(
            Tempus::Moment(Doy::from_ymd(2023, 3, 16)),
            date_matcher(today, From, "March 16th 2023").unwrap(),
        );
    }

    #[test]
    fn parse_week() {
        let today = Doy::from_ymd(2023, 3, 17);

        assert_eq!(
            Tempus::Interval(Doy::from_ymd(2023, 3, 27), Doy::from_ymd(2023, 4, 3)),
            date_matcher(today, From, "2023-W13").unwrap(),
        );
        assert_eq!(
            Tempus::Interval(Doy::from_ymd(2020, 12, 21), Doy::from_ymd(2020, 12, 28)),
            date_matcher(today, From, "Woche 2020-52").unwrap(),
        );

        assert_eq!(
            Tempus::Interval(Doy::from_ymd(2020, 12, 21), Doy::from_ymd(2020, 12, 28)),
            date_matcher(today, From, "KW 20/52").unwrap(),
        );
    }
}
