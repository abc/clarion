# cstime

A library to simplify working with CSTime software date/time formats by 
enabling simple translation to and from the CSTime formats into the formats
used/provided by [time](https://crates.io/crates/time).

CSTime formats are all based on integral numbers, represented within this 
library as `i32`s. Dates are specified as the number of days between a given
date and the 28th of December, 1800. Times are specified as the number of 
centiseconds between a given time and midnight.

A standard error format is also specified, named `CSErr`, which at present
is only used for out-of-range conversion errors (`CSErr::ConversionOverflowed`).

## Usage

This library provides two types, `CSTime` and `CSDate` which represent 
time and date values, respectively. They can be created using the `new`
constructor or with the standard struct initialization syntax:

```rust
// Create a new CSTime value.
let cs_time = cstime::CSTime::new(5964000);
let cs_time2 = cstime::CSTime { time: 5964000 };

// Create a new CSDate value.
let cs_date = cstime::CSDate::new(80727);
let cs_date2 = cstime::CSDate { date: 80727 };
```

These objects can be freely converted to/from `time` compatible formats
using the `from()`/`try_from()` and/or the `into()`/`try_into()` functions.

```rust
// Convert `Date` into `CSDate` using `from()`.
let date = time::macros::date!(2022-01-05);
let cs_date = cstime::CSDate::from(date);
assert_eq!(cs_date.date, 80727);

// Convert `CSDate` into `Date` using `try_from()`.
let cs_date = cstime::CSDate::new(80727);
let date = time::Date::try_from(cs_date)
    .expect("The input value produce a valid date between Date::MAX and Date::MIN.");
assert_eq!(date, time::macros::date!(2022-01-05));

// Convert `CSTime` into `Time` using `into()`.
let cs_time = cstime::CSTime::new(5964000);
let time: time::Time = cs_time.into();
assert_eq!(time, time::macros::time!(16:34:00))
```

The conversion is fully reversible:

```rust
let cs_date = cstime::CSDate::new(80727);
let date: time::Date = cs_date.try_into().unwrap();
let another_cs_date: cstime::CSDate = date.into();
assert_eq!(another_cs_date, cs_date);
```

The raw `i32` value can be extracted out of a `CSTime`/`CSDate` value by 
directly accessing the `time` or `date` values of the respective struct.

## License

This project is licensed under either of

- <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or
- <a href="LICENSE-MIT">MIT license</a>

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in cstime by you, as defined in the Apache-2.0 license, shall be 
dual licensed as above, without any additional terms or conditions.