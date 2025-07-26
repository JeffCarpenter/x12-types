#[macro_export]
macro_rules! parse_opt_into {
    ($rest:expr, $out:expr, $field:ident, $parser:path) => {{
        let (r, val) = nom::combinator::opt($parser).parse($rest)?;
        $out.$field = val;
        r
    }};
}

#[macro_export]
macro_rules! parse_many_into {
    ($rest:expr, $out:expr, $field:ident, $parser:path) => {{
        let (r, val) = nom::multi::many0($parser).parse($rest)?;
        $out.$field = val;
        r
    }};
}
