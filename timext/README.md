## timext

[![Build Status][action-badge]][action-url]
[![Crate Docs][docs-badge]][docs-url]
[![Crate Version][crates-badge]][crates-url]
[![Crate Coverage][coverage-badge]][coverage-url]

**Also check out other `xwde` projects [here](https://github.com/xwde).**

[action-badge]: https://img.shields.io/github/actions/workflow/status/xwde/time/build.yaml?branch=main&label=build&logo=github&style=flat-square
[action-url]: https://github.com/xwde/time/actions/workflows/build.yaml
[crates-badge]: https://img.shields.io/crates/v/timext.svg?logo=rust&style=flat-square
[crates-url]: https://crates.io/crates/timext
[docs-badge]: https://img.shields.io/docsrs/timext?logo=Docs.rs&style=flat-square
[docs-url]: http://docs.rs/timext
[coverage-badge]: https://img.shields.io/codecov/c/github/xwde/time?logo=codecov&logoColor=white&style=flat-square
[coverage-url]: https://app.codecov.io/gh/xwde/time

The collection of [time-rs/time](https://github.com/time-rs/time/) extensions
for calendar arithmetics, incomplete formats handling, imprecise time, and other
things `time` crate is not intended for.

For details, see the [main](https://crates.io/crates/timext) crate.

- Introduces `CalendarDuration` and extends `time::Date`,
  `time::PrimitiveDateTime` and `time::OffsetDateTime` with several methods to
  enable arithmetic operations related to months and years. Additionally,
  attaches conversion methods to `i64` and `f64` to improve ease of use.
  **Enabled with feature `month`.**

```rust
use time::{Date, Month::*};
use timext::ext::NumericCalendarDuration;

fn main() {
    let d0 = Date::from_calendar_date(2023, January, 31);
    let d1 = Date::from_calendar_date(2023, February, 28);
    assert_eq!(d0.unwrap() + 1.months(), d1.unwrap());

    let d0 = Date::from_calendar_date(2024, February, 29);
    let d1 = Date::from_calendar_date(2025, February, 28);
    assert_eq!(d0.unwrap() + 1.years(), d1.unwrap());
}
```

- Implements its own `time::Time`, `time::Date`, `time::PrimitiveDateTime`, and
  `time::OffsetDateTime` types, that are convertable from/to original, but allow
  incomplete time formats e.g. `xx:24:xx.845`, `1998-xx-02` or `2016-08 14:xx`.
  **Enabled with feature `parts`.**

> **Warning** : `parsing` & `formatting` are not yet implemented.

```rust
use time::{Date, Month::*};
use timext::partial::{Partial, PartDate};

fn main() {
    let d0 = Date::from_calendar_date(2023, January, 28);
    let d1 = PartDate::from_calendar_date(None, None, Some(28));
    let d1 = d1.unwrap();

    let d1 = d1.replace_year(Some(2023)).unwrap();
    let d1 = d1.replace_month(Some(January)).unwrap();
    assert_eq!(d0.unwrap(), d1.into_complete().unwrap());
}
```

#### Crates

- [time-rs/time](https://github.com/time-rs/time/)
