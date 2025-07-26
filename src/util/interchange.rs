use crate::util::Parser;
use crate::v004010::*;
use nom::{multi::many0, IResult, Parser as _};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct Transmission<T> {
    pub isa: ISA,
    pub functional_group: Vec<FunctionalGroup<T>>,
    pub iea: IEA,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct FunctionalGroup<T> {
    pub gs: GS,
    pub segments: Vec<T>,
    pub ge: GE,
}

impl<'a, T> Parser<&'a str, Transmission<T>, nom::error::Error<&'a str>> for Transmission<T>
where
    T: for<'b> Parser<&'b str, T, nom::error::Error<&'b str>> + Default,
{
    fn parse(input: &'a str) -> IResult<&'a str, Transmission<T>> {
        Self::parse_with(input, false)
    }
}

impl<T> Transmission<T>
where
    T: for<'a> Parser<&'a str, T, nom::error::Error<&'a str>> + Default,
{
    fn parse_with(input: &str, allow_empty: bool) -> IResult<&str, Self> {
        let mut output = Transmission::default();
        let (mut rest, isa) = ISA::parse(input)?;
        output.isa = isa;
        while let Ok((r, gs)) = GS::parse(rest) {
            let (r, segments) = many0(T::parse).parse(r)?;
            let (r, ge) = GE::parse(r)?;
            output
                .functional_group
                .push(FunctionalGroup { gs, segments, ge });
            rest = r;
        }
        if !allow_empty && output.functional_group.is_empty() {
            return Err(nom::Err::Error(nom::error::Error::new(
                rest,
                nom::error::ErrorKind::Tag,
            )));
        }
        let (rest, iea) = IEA::parse(rest)?;
        output.iea = iea;
        Ok((rest, output))
    }

    /// Parses an interchange that may omit all functional groups.
    pub fn parse_allow_empty(input: &str) -> IResult<&str, Self> {
        Self::parse_with(input, true)
    }
}

impl<T: Display> Display for Transmission<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.isa)?;
        for fg in &self.functional_group {
            write!(f, "{}", fg.gs)?;
            for segment in &fg.segments {
                write!(f, "{}", segment)?;
            }
            write!(f, "{}", fg.ge)?;
        }
        write!(f, "{}", self.iea)
    }
}
