use chrono::{Datelike, NaiveDate};

pub fn time_info(time: &str) -> String {
    let date = NaiveDate::parse_from_str(time, "%Y-%m-%d").unwrap();

    // 当前日期是第几周，周几（周一为1，周日为7）
    let week = date.iso_week().week();
    let weekday = match date.weekday().num_days_from_sunday() {
        0 => 7, // 如果是周日，返回7
        n => n, // 其他则返回相应的星期几
    };

    // 当前日期是本年中的第几天
    let day_of_year = date.ordinal();

    // 计算当年还剩多少天，确保不包括当天
    let end_of_year = NaiveDate::from_ymd(date.year(), 12, 31);
    let days_left = end_of_year.signed_duration_since(date).num_days() - 1; // 不包含当天

    // 距离春节（2025年1月29日）还有多少天，确保不包含当天
    let spring_festival = NaiveDate::from_ymd(2025, 1, 29);
    let days_to_spring_festival = spring_festival.signed_duration_since(date).num_days() - 1; // 不包含当天

    // 距离下次A股开盘（2025年1月20日）还有多少天，确保不包含当天
    let stock_opening = NaiveDate::from_ymd(2025, 1, 20);
    let days_to_stock_opening = stock_opening.signed_duration_since(date).num_days() - 1; // 不包含当天

    // 返回结果
    format!(
        "{},{},{},{},{},{}",
        week, weekday, day_of_year, days_left, days_to_spring_festival, days_to_stock_opening
    )
}
