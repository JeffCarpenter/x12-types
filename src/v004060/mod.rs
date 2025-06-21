//! v004060 represents select CBP truck specifications.

use crate::util::interchange::FunctionalGroup;
pub use crate::util::interchange::Transmission;
use crate::util::Parser;
use nom::{combinator::opt, multi::many0, IResult, Parser as _};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use x12_types_macros::DisplayX12;

use crate::v004010::*;

#[cfg(test)]
mod test_350;
#[cfg(test)]
mod test_355;
#[cfg(test)]
mod test_358;
#[cfg(test)]
pub mod test_util;

/// Truck Import Manifest Customs Status Information (TS350)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, DisplayX12)]
pub struct TS350TruckCBPCustomsStatusInformation {
    pub st: ST,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m10: Option<M10>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub p4: Vec<P4>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub v9: Vec<V9>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nm1: Vec<NM1>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub vid: Vec<VID>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub m7: Vec<M7>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub k1: Vec<K1>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub n9: Vec<N9>,
    pub se: SE,
}

impl<'a> Parser<&'a str, TS350TruckCBPCustomsStatusInformation, nom::error::Error<&'a str>>
    for TS350TruckCBPCustomsStatusInformation
{
    fn parse(input: &'a str) -> IResult<&'a str, TS350TruckCBPCustomsStatusInformation> {
        let mut output = TS350TruckCBPCustomsStatusInformation::default();
        let (rest, st) = ST::parse(input)?;
        output.st = st;
        let (rest, m10) = opt(M10::parse).parse(rest)?;
        output.m10 = m10;
        let (rest, p4) = many0(P4::parse).parse(rest)?;
        output.p4 = p4;
        let (rest, v9) = many0(V9::parse).parse(rest)?;
        output.v9 = v9;
        let (rest, nm1) = many0(NM1::parse).parse(rest)?;
        output.nm1 = nm1;
        let (rest, vid) = many0(VID::parse).parse(rest)?;
        output.vid = vid;
        let (rest, m7) = many0(M7::parse).parse(rest)?;
        output.m7 = m7;
        let (rest, k1) = many0(K1::parse).parse(rest)?;
        output.k1 = k1;
        let (rest, n9) = many0(N9::parse).parse(rest)?;
        output.n9 = n9;
        let (rest, se) = SE::parse(rest)?;
        output.se = se;
        Ok((rest, output))
    }
}

/// Truck Import Manifest – Acceptance/Rejection (TS355)
#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct TS355TruckAcceptanceRejection {
    pub st: ST,
    pub m10: M10,
    pub ak1: AK1,
    pub ak9: AK9,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ak2: Vec<AK2>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ak5: Vec<AK5>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ak3: Vec<AK3>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ak4: Vec<AK4>,
    pub se: SE,
}

impl<'a> Parser<&'a str, TS355TruckAcceptanceRejection, nom::error::Error<&'a str>>
    for TS355TruckAcceptanceRejection
{
    fn parse(input: &'a str) -> IResult<&'a str, TS355TruckAcceptanceRejection> {
        let mut output = TS355TruckAcceptanceRejection::default();
        let (rest, st) = ST::parse(input)?;
        output.st = st;
        let (rest, m10) = M10::parse(rest)?;
        output.m10 = m10;
        let (rest, ak1) = AK1::parse(rest)?;
        output.ak1 = ak1;
        let (rest, ak9) = AK9::parse(rest)?;
        output.ak9 = ak9;
        let (rest, ak2) = many0(AK2::parse).parse(rest)?;
        output.ak2 = ak2;
        let (rest, ak5) = many0(AK5::parse).parse(rest)?;
        output.ak5 = ak5;
        let (rest, ak3) = many0(AK3::parse).parse(rest)?;
        output.ak3 = ak3;
        let (rest, ak4) = many0(AK4::parse).parse(rest)?;
        output.ak4 = ak4;
        let (rest, se) = SE::parse(rest)?;
        output.se = se;
        Ok((rest, output))
    }
}

/// Truck Import Manifest – Consist/Trip Information (TS358)
#[derive(Debug, Default, Clone, Serialize, Deserialize, DisplayX12)]
pub struct TS358TruckConsistTripInformation {
    pub st: ST,
    pub m10: M10,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub m7: Vec<M7>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nm1: Option<NM1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dmg: Option<DMG>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#ref: Vec<REF>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub n3: Vec<N3>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n4: Option<N4>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub p4: Vec<P4>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub vid: Vec<VID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m13: Option<M13>,
    pub se: SE,
}

impl<'a> Parser<&'a str, TS358TruckConsistTripInformation, nom::error::Error<&'a str>>
    for TS358TruckConsistTripInformation
{
    fn parse(input: &'a str) -> IResult<&'a str, TS358TruckConsistTripInformation> {
        let mut output = TS358TruckConsistTripInformation::default();
        let (rest, st) = ST::parse(input)?;
        output.st = st;
        let (rest, m10) = M10::parse(rest)?;
        output.m10 = m10;
        let (rest, m7) = many0(M7::parse).parse(rest)?;
        output.m7 = m7;
        let (rest, nm1) = opt(NM1::parse).parse(rest)?;
        output.nm1 = nm1;
        let (rest, dmg) = opt(DMG::parse).parse(rest)?;
        output.dmg = dmg;
        let (rest, r_ref) = many0(REF::parse).parse(rest)?;
        output.r#ref = r_ref;
        let (rest, n3) = many0(N3::parse).parse(rest)?;
        output.n3 = n3;
        let (rest, n4) = opt(N4::parse).parse(rest)?;
        output.n4 = n4;
        let (rest, p4) = many0(P4::parse).parse(rest)?;
        output.p4 = p4;
        let (rest, vid) = many0(VID::parse).parse(rest)?;
        output.vid = vid;
        let (rest, m13) = opt(M13::parse).parse(rest)?;
        output.m13 = m13;
        let (rest, se) = SE::parse(rest)?;
        output.se = se;
        Ok((rest, output))
    }
}
