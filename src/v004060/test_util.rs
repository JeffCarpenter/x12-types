#[cfg(test)]
/// Wraps transactions in a single-group interchange for unit tests.
///
/// The helper only supports one functional group and reuses the provided
/// control number for both GE and IEA segments.
pub fn wrap(prefix: &str, body: &str, ctrl: &str) -> String {
    format!(
        "ISA*00*          *00*          *ZZ*TEST*ZZ*DEST*250101*0100*U*00400*{ctrl}*0*T*>~{prefix}{body}GE*1*1~IEA*1*{ctrl}~"
    )
}
