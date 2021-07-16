pub fn days_between_dates(date1: String, date2: String) -> i32 {
    let date1 = from_1971_01_01(date1);
    let date2 = from_1971_01_01(date2);

    (date1 - date2).abs()
}

pub fn from_1971_01_01(date: String) -> i32 {
    let m_d = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    let date = date
        .split('-')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();

    let mut days = date[2] - 1 + m_d[date[1] as usize - 1];
    if date[0] % 4 == 0 && date[0] != 2100 && date[1] > 2 {
        days += 1;
    }
    days += 365 * (date[0] - 1971) + (date[0] - 1969) / 4;

    days
}

fn main() {
    let x = days_between_dates("2019-06-29".to_string(), "2019-06-30".to_string());
    println!("{}", x);

    let x = days_between_dates("2019-05-29".to_string(), "2019-06-30".to_string());
    println!("{}", x);

    let x = days_between_dates("2020-06-29".to_string(), "2020-06-15".to_string());
    println!("{}", x);
}