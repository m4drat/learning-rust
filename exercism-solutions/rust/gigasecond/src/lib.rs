use std::{ops::Add, time::Duration};

use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start.add(Duration::new(1_000_000_000, 0))
}
