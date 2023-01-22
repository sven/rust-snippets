# Rust Snippets

Disclaimer: The information that can be found here is what I've found on the
very helpful [The Rust Programming Language Forum](https://users.rust-lang.org/)
and on various great Internet sites. If something should be improved or is
wrong please file an issue or merge request.


## Derive Implementation Example

Location: [Derive Implementation Example](derive_impl)

Helpful sources:
  * [The Rust Reference - Procedural Macros](https://doc.rust-lang.org/reference/procedural-macros.html)
  * [The Rust Programming Language - How to Write a Custom derive Macro](https://doc.rust-lang.org/book/ch19-06-macros.html?highlight=proc-macro#how-to-write-a-custom-derive-macro)
  * [Crate syn Documentation](https://docs.rs/syn/latest/syn)
  * [developerlife - Guide to Rust procedural macros](https://developerlife.com/2022/03/30/rust-proc-macro)

This example implements an own derive which parses a struct and adds a new
method to show the fields metadata. Run the example by changing into the
`derive_demo` subdirectory and call `cargo run`.

Example usage as shown in the `derive_demo`:

```rust
use derive_lib::DeriveStruct;

#[derive(DeriveStruct)]
struct ParseMe {
    #[derive_struct_attr(name = "ElementString", no = 123)]
    _element_string: String,

    #[derive_struct_attr(name = "ElementU8")]
    _element_u8: u8,
}
```

This is parsed through the library `derive_lib` that has the special
`proc-macro = true` config in it's `Cargo.toml` which tells the compiler to
execute it during the compilation.

The output of the example will look like this:

```rust
Struct name: ParseMe
  * Field: _element_string
    * Attribute: name = Str(LitStr { token: "ElementString" })
    * Attribute: no = Int(LitInt { token: 123 })
  * Field: _element_u8
    * Attribute: name = Str(LitStr { token: "ElementU8" })
```

In the current implementation it can't modify the struct itself but only parse
it. If you need to modify the struct you'll have to wrap it in a macro like
this and adapt the parser:

```rust
my_macro! {
    struct ParseMe {
        #[derive_struct_attr(name = "Hello", attribute = "Hi")]
        abc: String,

        #[derive_struct_attr(i_am_a_number = 123)]
        def: u8,
    }
}
```
