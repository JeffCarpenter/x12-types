#![cfg(test)]

use crate::util::interchange::Transmission;
use crate::v004010::ISA;

#[test]
fn parse_allow_empty_ok() {
    let s = "ISA*00*          *00*          *ZZ*TEST*ZZ*DEST*250101*0100*U*00400*000000001*0*T*>~IEA*1*000000001~";
    let (rest, tx) = Transmission::<ISA>::parse_allow_empty(s).unwrap();
    assert!(rest.is_empty());
    assert!(tx.functional_group.is_empty());
    assert_eq!(tx.iea._02, "000000001");
}
