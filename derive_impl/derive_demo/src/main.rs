//! Shows how to use the DeriveStruct procedural macro

use derive_lib::DeriveStruct;

/// test structure for DeriveStruct struct parser
#[derive(DeriveStruct, Default)]
struct ParseMe {
    /// field that is a string
    #[derive_struct_attr(name = "ElementString", no = 123)]
    _element_string: String,

    /// field that is an u8
    #[derive_struct_attr(name = "ElementU8")]
    _element_u8: u8,
}

fn main() {
    let parse_me = ParseMe::default();
    parse_me.print_meta();
}
