use crate::util::interchange::Transmission;
use crate::v004010::{ISA, _309};

fn wrap(prefix: &str, body: &str, ctrl: &str) -> String {
    format!(
        "ISA*00*          *00*          *ZZ*TEST*ZZ*DEST*250101*0100*U*00400*{ctrl}*0*T*>~{prefix}{body}GE*1*1~IEA*1*{ctrl}~"
    )
}

#[test]
fn parse_allow_empty_ok() {
    let s = "ISA*00*          *00*          *ZZ*TEST*ZZ*DEST*250101*0100*U*00400*000000001*0*T*>~IEA*1*000000001~";
    let (rest, tx) = Transmission::<ISA>::parse_allow_empty(s).unwrap();
    assert!(rest.is_empty());
    assert!(tx.functional_group.is_empty());
    assert_eq!(tx.iea._02, "000000001");
}

#[test]
fn parse_allow_empty_group() {
    let body = "ST*309*0001~\
M10*AA*O*SG*1*VESSEL*320N*123*21*Y~\
P4*1803*20250101*1*M029~\
LX*1~\
SE*5*0001~";
    let msg = wrap("GS*SO*AA*BB*20250101*0100*1*X*004010~", body, "000000010");
    let (rest, tx) = Transmission::<_309>::parse_allow_empty(&msg).unwrap();
    assert!(rest.is_empty());
    assert_eq!(tx.functional_group.len(), 1);
    let segs = &tx.functional_group[0].segments[0];
    assert_eq!(segs.st._01, "309");
}
