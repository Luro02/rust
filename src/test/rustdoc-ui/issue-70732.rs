// compile-flags:--test --test-args --test-threads=1
// normalize-stdout-test: "src/test/rustdoc-ui" -> "$$DIR"
//
// The indentation would be stripped from a doc attribute in between normal doc comments
// (`//!` or `///`).
//
// https://github.com/rust-lang/rust/issues/70732

//! ```rust
//! let x = r#"
//!   bar();
//! "#;
//! assert_eq!(x, "\n  bar();\n");
//! ```

#![doc = " ```rust"]
#![doc = " let x = r#\""]
#![doc = "   bar();"]
#![doc = " \"#;"]
#![doc = " assert_eq!(x, \"\\n  bar();\\n\");"]
#![doc = " ```"]

//! ```rust
//! let x = r#"
#![doc = "   bar();"]
//! "#;
//! assert_eq!(x, "\n  bar();\n");
//! ```
