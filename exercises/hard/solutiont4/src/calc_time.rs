use chrono::{Datelike, NaiveDate, Weekday};

// 假设我们有一个法定节假日列表
const HOLIDAYS: &[&str] = &["2025-01-01", "2025-01-29"];

// 获取指定年份春节（正月初一）的日期
fn get_chinese_new_year_date(year: i32) -> NaiveDate {
    match year {
        2025 => NaiveDate::from_ymd_opt(2025, 1, 29).unwrap(),
        _ => panic!("Unsupported year for Chinese New Year calculation"),
    }
}

// 判断指定日期是否为 A 股开盘日
fn is_a_share_trading_day(date: NaiveDate) -> bool {
    let weekday = date.weekday();
    let date_str = date.format("%Y-%m-%d").to_string();

    // 排除周末和法定节假日
    weekday != Weekday::Sat && weekday != Weekday::Sun && !HOLIDAYS.contains(&date_str.as_str())
}

// 计算距离下一次 A 股开盘的天数
fn days_until_next_a_share_trading_day(date: NaiveDate) -> u32 {
    let mut days = 0;
    let mut next_date = date.succ_opt().unwrap();
    while !is_a_share_trading_day(next_date) {
        next_date = next_date.succ_opt().unwrap();
        days += 1;
    }
    days
}

// 检查是否为闰年
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

// 主函数，计算并返回所需的时间信息
pub fn time_info(time: &str) -> String {
    let date = NaiveDate::parse_from_str(time, "%Y-%m-%d").unwrap();
    let year = date.year();
    let is_leap = is_leap_year(year);
    let days_in_year = if is_leap { 366 } else { 365 };

    // 计算当前周是第几周
    let week_number = date.iso_week().week();
    // 计算周几
    let weekday = date.weekday().num_days_from_monday() + 1;
    // 计算当天是本年的第几天
    let day_of_year = date.ordinal();
    // 计算当年还剩多少天
    let days_left_in_year = days_in_year - day_of_year;

    // 获取春节日期
    let chinese_new_year_date = get_chinese_new_year_date(year);
    // 计算距离春节的天数
    let days_until_chinese_new_year = (chinese_new_year_date - date).num_days();
    let days_until_chinese_new_year = if days_until_chinese_new_year < 0 {
        0
    } else {
        days_until_chinese_new_year as u32
    };

    // 计算距离下一次 A 股开盘的天数
    let days_until_next_trading_day = days_until_next_a_share_trading_day(date);

    format!(
        "{},{},{},{},{},{}",
        week_number,
        weekday,
        day_of_year,
        days_left_in_year,
        days_until_chinese_new_year,
        days_until_next_trading_day
    )
}
