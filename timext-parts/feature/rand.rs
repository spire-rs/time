use rand::distributions::{Distribution, Standard};
use rand::Rng;

use crate::partial::{PartDate, PartOffsetDateTime, PartPrimitiveDateTime, PartTime, Partial};

impl Distribution<PartDate> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PartDate {
        PartDate::from_complete(Self.sample(rng))
    }
}

impl Distribution<PartTime> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PartTime {
        PartTime::from_complete(Self.sample(rng))
    }
}

impl Distribution<PartPrimitiveDateTime> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PartPrimitiveDateTime {
        PartPrimitiveDateTime::from_complete(Self.sample(rng))
    }
}

impl Distribution<PartOffsetDateTime> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PartOffsetDateTime {
        PartOffsetDateTime::from_complete(Self.sample(rng))
    }
}
