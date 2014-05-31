/*
You are given the following information, but you may prefer to do some research for yourself.

    1 Jan 1900 was a Monday.
    Thirty days has September,
    April, June and November.
    All the rest have thirty-one,
    Saving February alone,
    Which has twenty-eight, rain or shine.
    And on leap years, twenty-nine.
    A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.

How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?
*/

enum Day {
    Monday=1,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday
}

enum Month {
    Jan=1,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec
}

fn num_days(month: Month, year: int) -> int
{
    match month {
        Jan => 31,
        Feb => if is_leap(year) { 29 } else { 28 },
        Mar => 31,
        Apr => 30,
        May => 31,
        Jun => 30,
        Jul => 31,
        Aug => 31,
        Sep => 30,
        Oct => 31,
        Nov => 30,
        Dec => 31
    }
}

fn next_month(month: Month) -> Month
{
    match month {
        Jan => Feb,
        Feb => Mar,
        Mar => Apr,
        Apr => May,
        May => Jun,
        Jun => Jul,
        Jul => Aug,
        Aug => Sep,
        Sep => Oct,
        Oct => Nov,
        Nov => Dec,
        Dec => Jan
    }
}

fn next_day(day: Day) -> Day
{
    match day {
        Monday => Tuesday,
        Tuesday => Wednesday,
        Wednesday => Thursday,
        Thursday => Friday,
        Friday => Saturday,
        Saturday => Sunday,
        Sunday => Monday
    }
}

fn is_leap(year: int) -> bool
{
    match year % 100 {
        0 => 0 == year % 400,
        _ => 0 == year % 4
    }
}

struct Date {
    date: int,
    day:  Day,
    month: Month,
    year: int
}

fn incrementDay(day: &mut Day) -> () {
    *day = next_day(*day);
}

fn incrementMonth(month: &mut Month, year: &mut int) -> ()
{
    match *month {
        Dec => *year += 1,
        _ => ()
    }
    *month = next_month(*month);
}

fn increment(date: &mut Date) -> ()
{
    incrementDay(&mut date.day);
    // reached end of month, roll over.
    if date.date == num_days(date.month, date.year) {
        date.date = 1;
        incrementMonth(&mut date.month, &mut date.year);
    } else {
        date.date += 1;
    }
}

fn terminate(date: &Date) -> bool
{
    match *date {
        Date {date: 31, day: _, month: Dec, year: 2000} => true,
        _ => false
    }
}

fn first_of_month_sunday(date: &Date) -> bool
{
    match *date {
        Date {date: 1, day: Sunday, month: _, year: _} => true,
        _ => false
    }
}

fn main()
{
    let mut dt: Date = Date{date:1, day: Monday, month: Jan, year: 1900};

    let mut i = 0;
    while true {
        if terminate(&mut dt) {
            break;
        }
        if first_of_month_sunday(&mut dt) {
            println!("{} {} {} {}", dt.date, dt.day as int, dt.month as int, dt.year);
            i += 1;
        }
        increment(&mut dt);
    }

    println!("{:d}", i);

}
