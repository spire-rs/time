use time::{Date, OffsetDateTime, PrimitiveDateTime, Time};

use crate::partial::{PartDate, PartOffsetDateTime, PartPrimitiveDateTime, PartTime, Partial};

/// Converts the complete timestamp into the partial one.
pub trait IntoPartial {
    /// The partial timestamp type that corresponds to the complete timestamp.
    type Partial: Partial;

    /// Converts the complete timestamp into the partial one.
    fn into_partial(self) -> Self::Partial;
}

impl IntoPartial for Time {
    type Partial = PartTime;

    fn into_partial(self) -> Self::Partial {
        self.into()
    }
}

impl IntoPartial for Date {
    type Partial = PartDate;

    fn into_partial(self) -> Self::Partial {
        self.into()
    }
}

impl IntoPartial for PrimitiveDateTime {
    type Partial = PartPrimitiveDateTime;

    fn into_partial(self) -> Self::Partial {
        self.into()
    }
}

impl IntoPartial for OffsetDateTime {
    type Partial = PartOffsetDateTime;

    fn into_partial(self) -> Self::Partial {
        self.into()
    }
}
