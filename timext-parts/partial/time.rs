use time::Time;

use crate::error::{NoComponent, PartRange};
use crate::partial::Partial;

/// An `InTime` struct represents an incomplete [Time] struct.
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct PartTime {
    hour: Option<u8>,
    minute: Option<u8>,
    second: Option<u8>,
    nanosecond: Option<u32>,
}

impl PartTime {
    /// Attempts to create a `InTime` from the hour, minute, and second.
    pub fn from_hms(
        hour: Option<u8>,
        minute: Option<u8>,
        second: Option<u8>,
    ) -> Result<Self, PartRange> {
        Self::from_hms_nano(hour, minute, second, Some(0))
    }

    /// Attempts to create a `InTime` from the hour, minute, second, and millisecond.
    pub fn from_hms_milli(
        hour: Option<u8>,
        minute: Option<u8>,
        second: Option<u8>,
        millisecond: Option<u16>,
    ) -> Result<Self, PartRange> {
        let nanosecond = millisecond.map(|ms| ms as u32 * 1_000_000);
        Self::from_hms_nano(hour, minute, second, nanosecond)
    }

    /// Attempt to create a `InTime` from the hour, minute, second, and microsecond.
    pub fn from_hms_micro(
        hour: Option<u8>,
        minute: Option<u8>,
        second: Option<u8>,
        microsecond: Option<u32>,
    ) -> Result<Self, PartRange> {
        let nanosecond = microsecond.map(|ms| ms * 1_000);
        Self::from_hms_nano(hour, minute, second, nanosecond)
    }

    /// Attempt to create a `InTime` from the hour, minute, second, and nanosecond.
    pub fn from_hms_nano(
        hour: Option<u8>,
        minute: Option<u8>,
        second: Option<u8>,
        nanosecond: Option<u32>,
    ) -> Result<Self, PartRange> {
        // TODO: Soundness.
        Ok(Self::from_hms_nano_unchecked(
            hour, minute, second, nanosecond,
        ))
    }

    /// Creates a `InTime` from its components.
    fn from_hms_nano_unchecked(
        hour: Option<u8>,
        minute: Option<u8>,
        second: Option<u8>,
        nanosecond: Option<u32>,
    ) -> Self {
        Self {
            hour,
            minute,
            second,
            nanosecond,
        }
    }

    /// Returns the clock hour.
    pub fn hour(self) -> Option<u8> {
        self.hour
    }

    /// Returns the minute within the hour.
    pub fn minute(self) -> Option<u8> {
        self.minute
    }

    /// Returns the second within the minute.
    pub fn second(self) -> Option<u8> {
        self.second
    }

    /// Returns the milliseconds within the second.
    pub fn millisecond(self) -> Option<u16> {
        self.nanosecond.map(|ns| (ns / 1_000_000) as _)
    }

    /// Returns the microseconds within the second.
    pub fn microsecond(self) -> Option<u32> {
        self.nanosecond.map(|ns| (ns / 1_000) as _)
    }

    /// Returns the nanoseconds within the second.
    pub fn nanosecond(self) -> Option<u32> {
        self.nanosecond
    }

    /// Replaces the clock hour.
    pub fn replace_hour(self, hour: Option<u8>) -> Result<Self, PartRange> {
        Self::from_hms_nano(hour, self.minute, self.minute, self.nanosecond)
    }

    /// Replaces the minutes within the hour.
    pub fn replace_minute(self, minute: Option<u8>) -> Result<Self, PartRange> {
        Self::from_hms_nano(self.hour, minute, self.minute, self.nanosecond)
    }

    /// Replaces the seconds within the minute.
    pub fn replace_second(self, second: Option<u8>) -> Result<Self, PartRange> {
        Self::from_hms_nano(self.hour, self.minute, second, self.nanosecond)
    }

    /// Replaces the milliseconds within the second.
    pub fn replace_millisecond(self, millisecond: Option<u16>) -> Result<Self, PartRange> {
        Self::from_hms_milli(self.hour, self.minute, self.minute, millisecond)
    }

    /// Replaces the microseconds within the second.
    pub fn replace_microsecond(self, microsecond: Option<u32>) -> Result<Self, PartRange> {
        Self::from_hms_micro(self.hour, self.minute, self.minute, microsecond)
    }

    /// Replaces the nanoseconds within the second.
    pub fn replace_nanosecond(self, nanosecond: Option<u32>) -> Result<Self, PartRange> {
        Self::from_hms_nano(self.hour, self.minute, self.minute, nanosecond)
    }
}

impl Partial for PartTime {
    type Complete = Time;

    fn from_complete(complete: Self::Complete) -> Self {
        let h = Some(complete.hour());
        let m = Some(complete.minute());
        let s = Some(complete.second());
        let n = Some(complete.nanosecond());
        Self::from_hms_nano_unchecked(h, m, s, n)
    }

    fn into_complete(self) -> Result<Self::Complete, PartRange> {
        let f = |name: &'static str| -> PartRange { NoComponent::new(name).into() };

        let h = self.hour().ok_or_else(|| f("hour"))?;
        let m = self.minute().ok_or_else(|| f("minute"))?;
        let s = self.second().ok_or_else(|| f("second"))?;
        let n = self.nanosecond().ok_or_else(|| f("nanosecond"))?;

        let time = Self::Complete::from_hms_nano(h, m, s, n);
        time.map_err(|e| e.into())
    }

    fn with_fallback(self, fallback: Self::Complete) -> Result<Self, PartRange> {
        let h = Some(self.hour.unwrap_or(fallback.hour()));
        let m = Some(self.minute.unwrap_or(fallback.minute()));
        let s = Some(self.second.unwrap_or(fallback.second()));
        let n = Some(self.nanosecond.unwrap_or(fallback.nanosecond()));
        Self::from_hms_nano(h, m, s, n)
    }
}

impl From<Time> for PartTime {
    fn from(time: Time) -> Self {
        Self::from_complete(time)
    }
}

impl TryFrom<PartTime> for Time {
    type Error = PartRange;

    fn try_from(time: PartTime) -> Result<Self, Self::Error> {
        time.into_complete()
    }
}
