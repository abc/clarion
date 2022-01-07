# cstime

A library to simplify working with CSTime software date/time formats by 
enabling simple translation to and from the CSTime formats into the formats
used/provided by [time](https://crates.io/crates/time).

## Usage

This library provides two types, `CSTime` and `CSDate` which represent 
time and date values, respectively. They can be created using the `new`
constructor: 

```rust
// Create a new CSTime value.
var cs_time = CSTime::new(5964000);

// Create a new CSDate value. Note that CSDate::new returns a 
// Result<CSDate, String>, because the input can be out of range.
var cs_date = CSDate::new(80727).unwrap();
```

These objects can be freely converted to/from `time` compatible formats
using the `from()` and `into()` functions.

```rust
var cs_time = CSTime::new(5964000);
var cs_date = CSDate::new(80727).unwrap();
var time: time::Time = cs_time.into();
var date: time::Date::from(cs_date);
```

The conversion is fully reversible:

```rust
var cs_date = CSDate::new(80727).unwrap();
var date: time::Date = cs_date.into();
var another_cs_date: CSDate = date.into();
```

The raw `i32` value can be extracted out of a `CSTime`/`CSDate` value using
the `time()` or `date()` functions, respectively.

## License

This project is licensed under either of

- <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or
- <a href="LICENSE-MIT">MIT license</a>

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in cstime by you, as defined in the Apache-2.0 license, shall be 
dual licensed as above, without any additional terms or conditions.