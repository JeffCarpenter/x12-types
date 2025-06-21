use crate::util::Parser;
use crate::v004060::test_util::wrap;
use crate::v004060::{TS350TruckCBPCustomsStatusInformation, Transmission};

#[test]
fn parse_350_minimal() {
    let body = "ST*350*0001~\
M10*AA~\
P4*1234*20250101~\
V9*IC~\
NM1*PE~\
VID*AA**BB~\
M7*S1~\
K1*REM~\
N9*BM*123~\
SE*10*0001~";
    let s = wrap(
        "GS*AU*TEST*DEST*20250101*0100*1*X*004060~",
        body,
        "000000001",
    );
    let (rest, obj) = Transmission::<TS350TruckCBPCustomsStatusInformation>::parse(s).unwrap();
    assert!(rest.is_empty());
    let segs = &obj.functional_group[0].segments[0];
    assert_eq!(segs.st._01, "350");
    assert!(segs.m10.is_some());
    assert!(!segs.p4.is_empty());
}
