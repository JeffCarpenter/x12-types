use crate::util::Parser;
use crate::v004060::{TS355TruckAcceptanceRejection, Transmission};

#[test]
fn parse_355_minimal() {
    let s = "ISA*00*          *00*          *ZZ*TEST*ZZ*DEST*250101*0100*U*00400*000000002*0*T*>~\
GS*AQ*TEST*DEST*20250101*0100*1*X*004060~\
ST*355*0001~\
M10*AAA~\
AK1*PO*1~\
AK9*A*1*1*1~\
AK2*355*0001~\
AK5*A~\
AK3*ST*1~\
AK4*1**1~\
SE*9*0001~\
GE*1*1~\
IEA*1*000000002~";
    let (rest, obj) = Transmission::<TS355TruckAcceptanceRejection>::parse(s).unwrap();
    assert!(rest.is_empty());
    let segs = &obj.functional_group[0].segments[0];
    assert_eq!(segs.st._01, "355");
    assert_eq!(segs.ak2.len(), 1);
    assert_eq!(segs.ak5.len(), 1);
}
