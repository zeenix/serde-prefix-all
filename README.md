<p align="center">
  <a href="https://github.com/zeenix/serde-prefix-all/actions/workflows/rust.yml">
    <img alt="Build Status" src="https://github.com/zeenix/serde-prefix-all/actions/workflows/rust.yml/badge.svg">
  </a>
  <a href="https://docs.rs/serde-prefix-all/">
    <img alt="API Documentation" src="https://docs.rs/serde-prefix-all/badge.svg">
  </a>
  <a href="https://crates.io/crates/serde-prefix-all">
    <img alt="crates.io" src="https://img.shields.io/crates/v/serde-prefix-all">
  </a>
</p>

<h1 align="center">serde-prefix-all</h1>

# Serde Prefix All

A small extension to serde that will allow you to use the macro `#[prefix_all("myprefix_")`. The
macro will prefix each field in a struct or variant in an enum in the serialized format, with the
prefix of your choice.

Behind the doors it's using `#[serde(rename = "...")]` to rename each field/variant with the prefix
defined in prefix_all.

## Usage

```rust
use serde::{Serialize, Deserialize};
use serde_prefix_all::prefix_all;

#[prefix_all("test_")]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

let point = Point { x: 1, y: 2 };
let serialized = serde_json::to_string(&point).unwrap();
let json = r#"{"test_x":1,"test_y":2}"#;
assert_eq!(serialized, json);
let deserialized: Point = serde_json::from_str(json).unwrap();
assert_eq!(point, deserialized);
```

## Background

This is a fork of [serde-prefix](https://github.com/jonathan-s/serde-prefix/pulls). The
`serde-prefix` crate was unmaintained for years and this fork is a continuation of the work started
by [jonathan-s](https://github.com/jonathan-s).
