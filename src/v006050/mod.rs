//! v006050 Custom CBP truck specifications.

use crate::util::Parser;

use crate::v004010::*;

#[cfg(test)]
mod test_309;

pub use crate::util::interchange::Transmission;
/// Customs Manifest Transaction Set (TS309)
pub use crate::v004010::_309 as TS309CBPCustomsManifest;
