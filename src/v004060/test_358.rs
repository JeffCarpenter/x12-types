use crate::util::Parser;
use crate::v004060::test_util::wrap;
use crate::v004060::{TS358TruckConsistTripInformation, Transmission};

#[test]
fn parse_358_minimal() {
    let body = "ST*358*0001~\
M10*AAA~\
M7*SEAL~\
NM1*PE~\
DMG**20250101~\
REF*AB*123~\
N3*ADDR1~\
N4*CITY*ST*12345~\
P4*1234*20250101~\
VID*1**A~\
M13*AA*BB**CC****DD*~\
SE*12*0001~";
    let s = wrap(
        "GS*AQ*TEST*DEST*20250101*0100*1*X*004060~",
        body,
        "000000003",
    );
    let (rest, obj) = Transmission::<TS358TruckConsistTripInformation>::parse(s).unwrap();
    assert!(rest.is_empty());
    let segs = &obj.functional_group[0].segments[0];
    assert_eq!(segs.st._01, "358");
    assert!(segs.nm1.is_some());
    assert_eq!(segs.p4.len(), 1);
}
