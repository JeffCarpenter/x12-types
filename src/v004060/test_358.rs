use crate::util::Parser;
use crate::v004060::{TS358TruckConsistTripInformation, Transmission};

#[test]
fn parse_358_minimal() {
    let s = "ISA*00*          *00*          *ZZ*TEST*ZZ*DEST*250101*0100*U*00400*000000003*0*T*>~\
GS*AQ*TEST*DEST*20250101*0100*1*X*004060~\
ST*358*0001~\
M10*AAA*O~\
P4*1234*20250101~\
SE*4*0001~\
GE*1*1~\
IEA*1*000000003~";
    let (rest, obj) = Transmission::<TS358TruckConsistTripInformation>::parse(s).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj.functional_group[0].segments[0].st._01, "358");
}
