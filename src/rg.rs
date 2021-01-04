use crate::{Rg, EmitterOrg};
use std::fmt;
use std::fmt::Formatter;

impl Rg {
    /// Creates a new RG object
    ///
    /// # Example
    /// ```
    /// use validbr::Rg;
    /// use validbr::UF::SP;
    /// use validbr::EmitterOrg::SSP;
    ///
    /// let rg = Rg::new("A15987B-X", SSP(SP));
    /// assert_eq!(rg, Rg { code: "A15987B-X".to_string(), emitter_org: SSP(SP) })
    /// ```
    pub fn new(code: &str, emitter_org: EmitterOrg) -> Rg {
        Rg {
            code: code.to_string(),
            emitter_org
        }
    }

    /// Creates a new RG object
    ///
    /// # Example
    /// ```
    /// use validbr::Rg;
    /// use validbr::UF::SP;
    /// use validbr::EmitterOrg::SSP;
    ///
    /// let rg = Rg::from_string("A15987B-X".to_string(), SSP(SP));
    /// assert_eq!(rg, Rg { code: "A15987B-X".to_string(), emitter_org: SSP(SP) })
    /// ```
    pub fn from_string(code: String, emitter_org: EmitterOrg) -> Rg {
        Rg {
            code,
            emitter_org
        }
    }
}

impl fmt::Display for Rg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.code, self.emitter_org)
    }
}