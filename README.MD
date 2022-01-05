# cstime

A library to simplify working with CSTime software date/time formats by 
enabling simple translation to and from the CSTime formats into the formats
used/provided by [time](https://crates.io/crates/time).

## Usage

### From CSTime formats to `time` formats

To convert a CSTime date into a `time::Date` format:

```rust
use cstime::cstime_to_date;
use time::macros::date;
let date = cstime_to_date(80727).unwrap();
assert_eq!(date, date!(2022-01-05));
```

To convert a CSTime time into a `time::Time` format:

```rust
use cstime::cstime_to_time;
use time::macros::time;
assert_eq!(cstime_to_time(5964000), time!(16:34:00));
```

### From `time` formats to CSTime formats

To convert a `time::Date` format into a CSTime date format:

```rust
use cstime::date_to_cstime;
use time::macros::date;
let date = date!(2021-10-07);
assert_eq!(date_to_cstime(date), 80637);
```

To convert a `time::Time` format into a CSTime time format:

```rust
use cstime::time_to_cstime;
use time::macros::time;
let time = time!(16:01:54.21);
assert_eq!(time_to_cstime(time), 5771421);
```

## License

This project is licensed under either of

- <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or
- <a href="LICENSE-MIT">MIT license</a>

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in cstime by you, as defined in the Apache-2.0 license, shall be 
dual licensed as above, without any additional terms or conditions.