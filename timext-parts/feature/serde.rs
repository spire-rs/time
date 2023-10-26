use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::partial::*;

impl Serialize for PartTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        todo!()
    }
}

impl<'de> Deserialize<'de> for PartTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PartTimeVisitor;

        impl<'de> Visitor<'de> for PartTimeVisitor {
            type Value = PartTime;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct `PartTime`")
            }

            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                todo!()
            }
        }

        deserializer.deserialize_tuple(1, PartTimeVisitor)
    }
}

impl Serialize for PartDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        todo!()
    }
}

impl<'de> Deserialize<'de> for PartDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PartDateVisitor;

        impl<'de> Visitor<'de> for PartDateVisitor {
            type Value = PartDate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct `PartDate`")
            }

            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                todo!()
            }
        }

        deserializer.deserialize_tuple(1, PartDateVisitor)
    }
}

impl Serialize for PartPrimitiveDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        todo!()
    }
}

impl<'de> Deserialize<'de> for PartPrimitiveDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PartPrimitiveDateTimeVisitor;

        impl<'de> Visitor<'de> for PartPrimitiveDateTimeVisitor {
            type Value = PartPrimitiveDateTime;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct `PartPrimitiveDateTime`")
            }

            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                todo!()
            }
        }

        deserializer.deserialize_tuple(1, PartPrimitiveDateTimeVisitor)
    }
}

impl Serialize for PartOffsetDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        todo!()
    }
}

impl<'de> Deserialize<'de> for PartOffsetDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PartOffsetDateTimeVisitor;

        impl<'de> Visitor<'de> for PartOffsetDateTimeVisitor {
            type Value = PartOffsetDateTime;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct `PartOffsetDateTime`")
            }

            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                todo!()
            }
        }

        deserializer.deserialize_tuple(1, PartOffsetDateTimeVisitor)
    }
}
