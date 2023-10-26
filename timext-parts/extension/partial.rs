use time::{Date, OffsetDateTime, PrimitiveDateTime, Time};

use crate::partial::{PartDate, PartOffsetDateTime, PartPrimitiveDateTime, PartTime, Partial};

/// TODO: Desc.
pub trait IntoPartial {
    /// TODO: Desc.
    type Partial: Partial;

    /// TODO: Desc.
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
