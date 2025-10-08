use crate::util::Parser;
use crate::v004060::{TS350TruckCBPCustomsStatusInformation, Transmission};

#[test]
fn parse_350_minimal() {
    let s = "ISA*00*          *00*          *ZZ*TEST*ZZ*DEST*250101*0100*U*00400*000000001*0*T*>~\
GS*AU*TEST*DEST*20250101*0100*1*X*004060~\
ST*350*0001~\
SE*2*0001~\
GE*1*1~\
IEA*1*000000001~";
    let (rest, obj) = Transmission::<TS350TruckCBPCustomsStatusInformation>::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].st._01, "350");
}
