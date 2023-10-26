use time::{Month, OffsetDateTime, PrimitiveDateTime, UtcOffset, Weekday};

use crate::error::{TryFromPartial, PartialVariant};
use crate::partial::{PartDate, PartPrimitiveDateTime, PartTime, Partial};

/// An [`PartOffsetDateTime`] struct represents a partial [`OffsetDateTime`] struct.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct PartOffsetDateTime {
    datetime: PartPrimitiveDateTime,
    offset: Option<UtcOffset>,
}

impl PartOffsetDateTime {
    /// Creates a [`PartOffsetDateTime`] from its components.
    pub fn new(datetime: PartPrimitiveDateTime, offset: Option<UtcOffset>) -> Self {
        Self { datetime, offset }
    }
}

impl PartOffsetDateTime {
    /// Replaces the offset.
    pub fn offset(self) -> Option<UtcOffset> {
        self.offset
    }

    /// Returns the date.
    pub fn date(self) -> PartDate {
        self.datetime.date()
    }

    /// Returns the year.
    pub fn year(self) -> Option<i32> {
        self.datetime.year()
    }

    /// Returns the month.
    pub fn month(self) -> Option<Month> {
        self.datetime.month()
    }

    /// Returns the weekday.
    pub fn weekday(self) -> Option<Weekday> {
        self.datetime.weekday()
    }

    /// Returns the day of the month.
    pub fn day(self) -> Option<u8> {
        self.datetime.day()
    }

    /// Returns the time.
    pub fn time(self) -> PartTime {
        self.datetime.time()
    }

    /// Returns the clock hour.
    pub fn hour(self) -> Option<u8> {
        self.datetime.hour()
    }

    /// Returns the minute within the hour.
    pub fn minute(self) -> Option<u8> {
        self.datetime.minute()
    }

    /// Returns the second within the minute.
    pub fn second(self) -> Option<u8> {
        self.datetime.second()
    }

    /// Returns the milliseconds within the second.
    pub fn millisecond(self) -> Option<u16> {
        self.datetime.millisecond()
    }

    /// Returns the microseconds within the second.
    pub fn microsecond(self) -> Option<u32> {
        self.datetime.microsecond()
    }

    /// Returns the nanoseconds within the second.
    pub fn nanosecond(self) -> Option<u32> {
        self.datetime.nanosecond()
    }
}

impl PartOffsetDateTime {
    /// Replaces the offset.
    pub fn replace_offset(self, offset: Option<UtcOffset>) -> Result<Self, TryFromPartial> {
        Ok(Self::new(self.datetime, offset))
    }

    /// Replaces the date and time.
    pub fn replace_date_time(self, datetime: PartPrimitiveDateTime) -> Result<Self, TryFromPartial> {
        Ok(Self::new(datetime, self.offset))
    }

    /// Replaces the date.
    pub fn replace_date(self, date: PartDate) -> Result<Self, TryFromPartial> {
        self.replace_date_time(self.datetime.replace_date(date)?)
    }

    /// Replaces the year.
    pub fn replace_year(self, year: Option<i32>) -> Result<Self, TryFromPartial> {
        self.replace_date_time(self.datetime.replace_year(year)?)
    }

    /// Replace the month of the year.
    pub fn replace_month(self, month: Option<Month>) -> Result<Self, TryFromPartial> {
        self.replace_date_time(self.datetime.replace_month(month)?)
    }

    /// Replaces the day of the month.
    pub fn replace_day(self, day: Option<u8>) -> Result<Self, TryFromPartial> {
        self.replace_date_time(self.datetime.replace_day(day)?)
    }

    /// Returns the time.
    pub fn replace_time(self, time: PartTime) -> Result<Self, TryFromPartial> {
        self.replace_date_time(self.datetime.replace_time(time)?)
    }

    /// Returns the clock hour.
    pub fn replace_hour(self, hour: Option<u8>) -> Result<Self, TryFromPartial> {
        self.replace_date_time(self.datetime.replace_hour(hour)?)
    }

    /// Returns the minute within the hour.
    pub fn replace_minute(self, minute: Option<u8>) -> Result<Self, TryFromPartial> {
        self.replace_date_time(self.datetime.replace_minute(minute)?)
    }

    /// Returns the second within the minute.
    pub fn replace_second(self, second: Option<u8>) -> Result<Self, TryFromPartial> {
        self.replace_date_time(self.datetime.replace_second(second)?)
    }

    /// Returns the milliseconds within the second.
    pub fn replace_millisecond(self, millisecond: Option<u16>) -> Result<Self, TryFromPartial> {
        self.replace_date_time(self.datetime.replace_millisecond(millisecond)?)
    }

    /// Returns the microseconds within the second.
    pub fn replace_microsecond(self, microsecond: Option<u32>) -> Result<Self, TryFromPartial> {
        self.replace_date_time(self.datetime.replace_microsecond(microsecond)?)
    }

    /// Returns the nanoseconds within the second.
    pub fn replace_nanosecond(self, nanosecond: Option<u32>) -> Result<Self, TryFromPartial> {
        self.replace_date_time(self.datetime.replace_nanosecond(nanosecond)?)
    }
}

impl Partial for PartOffsetDateTime {
    type Complete = OffsetDateTime;

    fn from_complete(complete: Self::Complete) -> Self {
        let dt = PrimitiveDateTime::new(complete.date(), complete.time());
        Self::new(
            PartPrimitiveDateTime::from_complete(dt),
            Some(complete.offset()),
        )
    }

    fn into_complete(self) -> Result<Self::Complete, TryFromPartial> {
        let error: TryFromPartial = PartialVariant::new("offset").into();
        let offset = self.offset.ok_or(error)?;
        Ok(self.datetime.into_complete()?.assume_offset(offset))
    }

    fn with_fallback(self, fallback: Self::Complete) -> Result<Self, TryFromPartial> {
        let dt = PrimitiveDateTime::new(fallback.date(), fallback.time());
        let dt = self.datetime.with_fallback(dt)?;
        let os = Some(self.offset.unwrap_or(fallback.offset()));
        Ok(Self::new(dt, os))
    }
}

impl From<OffsetDateTime> for PartOffsetDateTime {
    fn from(datetime: OffsetDateTime) -> Self {
        Self::from_complete(datetime)
    }
}

impl TryFrom<PartOffsetDateTime> for OffsetDateTime {
    type Error = TryFromPartial;

    fn try_from(datetime: PartOffsetDateTime) -> Result<Self, Self::Error> {
        datetime.into_complete()
    }
}
