use crate::util::Parser;
use crate::v004060::{Transmission, TS355TruckAcceptanceRejection};

#[test]
fn parse_355_minimal() {
    let s = "ISA*00*          *00*          *ZZ*TEST*ZZ*DEST*250101*0100*U*00400*000000002*0*T*>~\
GS*AQ*TEST*DEST*20250101*0100*1*X*004060~\
ST*355*0001~\
M10*AAA*O~\
AK1*PO*1~\
AK9*A*1*1*1~\
SE*6*0001~\
GE*1*1~\
IEA*1*000000002~";
    let (rest, obj) = Transmission::<TS355TruckAcceptanceRejection>::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].st._01, "355");
}
