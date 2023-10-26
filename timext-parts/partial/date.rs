use time::{Date, Month, Weekday};

use crate::error::{PartialVariant, TryFromPartial};
use crate::partial::{PartPrimitiveDateTime, PartTime, Partial};

/// An [`PartDate`] struct represents an partial [`Date`] struct.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct PartDate {
    year: Option<i32>,
    month: Option<Month>,
    day: Option<u8>,
}

impl PartDate {
    /// Attempts to create a [`PartDate`] from the year, month, and day.
    pub fn from_calendar_date(
        year: Option<i32>,
        month: Option<Month>,
        day: Option<u8>,
    ) -> Result<Self, TryFromPartial> {
        // TODO: Soundness.
        Ok(Self::from_calendar_date_unchecked(year, month, day))
    }

    /// Creates a [`PartDate`] from its components.
    fn from_calendar_date_unchecked(
        year: Option<i32>,
        month: Option<Month>,
        day: Option<u8>,
    ) -> Self {
        Self { year, month, day }
    }
}

impl PartDate {
    /// Returns the year.
    pub fn year(self) -> Option<i32> {
        self.year
    }

    /// Returns the month.
    pub fn month(self) -> Option<Month> {
        self.month
    }

    /// Returns the weekday.
    pub fn weekday(self) -> Option<Weekday> {
        self.into_complete().ok().map(|d| d.weekday())
    }

    /// Returns the day of the month.
    pub fn day(self) -> Option<u8> {
        self.day
    }
}

impl PartDate {
    /// Replaces the year.
    pub fn replace_year(self, year: Option<i32>) -> Result<Self, TryFromPartial> {
        Self::from_calendar_date(year, self.month(), self.day())
    }

    /// Replaces the month of the year.
    pub fn replace_month(self, month: Option<Month>) -> Result<Self, TryFromPartial> {
        Self::from_calendar_date(self.year(), month, self.day())
    }

    /// Replaces the day of the month.
    pub fn replace_day(self, day: Option<u8>) -> Result<Self, TryFromPartial> {
        Self::from_calendar_date(self.year(), self.month(), day)
    }
}

impl PartDate {
    /// Creates a `InPrimitiveDateTime` using the existing date and the provided `InTime`.
    pub fn with_time(self, time: PartTime) -> PartPrimitiveDateTime {
        PartPrimitiveDateTime::new(self, time)
    }
}

impl Partial for PartDate {
    type Complete = Date;

    fn from_complete(complete: Self::Complete) -> Self {
        let y = Some(complete.year());
        let m = Some(complete.month());
        let d = Some(complete.day());
        Self::from_calendar_date_unchecked(y, m, d)
    }

    fn into_complete(self) -> Result<Self::Complete, TryFromPartial> {
        let f = |name: &'static str| -> TryFromPartial { PartialVariant::new(name).into() };

        let y = self.year().ok_or_else(|| f("year"))?;
        let m = self.month().ok_or_else(|| f("month"))?;
        let d = self.day().ok_or_else(|| f("day"))?;

        let date = Self::Complete::from_calendar_date(y, m, d);
        date.map_err(|e| e.into())
    }

    fn with_fallback(self, fallback: Self::Complete) -> Result<Self, TryFromPartial> {
        let y = Some(self.year.unwrap_or(fallback.year()));
        let m = Some(self.month.unwrap_or(fallback.month()));
        let d = Some(self.day.unwrap_or(fallback.day()));
        Self::from_calendar_date(y, m, d)
    }
}

impl From<Date> for PartDate {
    fn from(date: Date) -> Self {
        Self::from_complete(date)
    }
}

impl TryFrom<PartDate> for Date {
    type Error = TryFromPartial;

    fn try_from(date: PartDate) -> Result<Self, Self::Error> {
        date.into_complete()
    }
}
