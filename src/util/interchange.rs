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
    T: Default + Parser<&'a str, T, nom::error::Error<&'a str>>,
{
    fn parse(input: &'a str) -> IResult<&'a str, Transmission<T>> {
        let mut output = Transmission::default();
        let (rest, isa) = ISA::parse(input)?;
        output.isa = isa;
        let (rest, gs) = GS::parse(rest)?;
        let (rest, segments) = many0(T::parse).parse(rest)?;
        let (rest, ge) = GE::parse(rest)?;
        let fg = FunctionalGroup { gs, segments, ge };
        output.functional_group.push(fg);
        let (rest, iea) = IEA::parse(rest)?;
        output.iea = iea;
        Ok((rest, output))
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
