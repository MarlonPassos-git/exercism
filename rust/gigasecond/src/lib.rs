use time::{Date, Duration, OffsetDateTime, PrimitiveDateTime, Time};

// Returns a DateTime one billion seconds after start.
pub fn after(start: PrimitiveDateTime) -> PrimitiveDateTime {
    let giga_second = Duration::seconds(1_000_000_000);
    return start + giga_second;
}

// how to get a current_primitive_date_time
pub fn current_primitive_date_time() -> PrimitiveDateTime {
    let local = OffsetDateTime::now_utc();
    let month = local.month();
    let day = local.day();
    let year = local.year();
    let hour = local.hour();
    let minute = local.minute();
    let second = local.second();

    let date = Date::from_calendar_date(year, month, day).expect("invalid date");
    let time = Time::from_hms(hour, minute, second).expect("invalid hour");

    return PrimitiveDateTime::new(date, time);
}
