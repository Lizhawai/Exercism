use time::PrimitiveDateTime as DateTime;
use time::Duration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let added_date = start + Duration::seconds(1000000000);
    added_date
}