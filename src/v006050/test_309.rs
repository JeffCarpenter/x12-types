use crate::util::Parser;
use crate::v006050::{TS309CBPCustomsManifest, Transmission};

#[test]
fn parse_309_alias() {
    let s = "ISA*00*          *00*          *ZZ*AA*ZZ*BB*250101*0100*U*00401*000000010*0*T*>~\
GS*SO*AA*BB*20250101*0100*1*X*006050~\
ST*309*0001~\
M10*AA*O*SG*1*VESSEL*320N*123*21*Y~\
P4*1803*20250101*1*M029~\
LX*1~\
SE*5*0001~\
GE*1*1~\
IEA*1*000000010~";
    let (rest, obj) = Transmission::<TS309CBPCustomsManifest>::parse(s).unwrap();
    assert!(rest.is_empty());
    let segs = &obj.functional_group[0].segments[0];
    assert_eq!(segs.st._01, "309");
    assert_eq!(segs.loop_p4.len(), 1);
}
