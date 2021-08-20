use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;


#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub enum VersionType {
    Release,
    Beta,
    Alpha
}

use VersionType::*;

impl VersionType {
    fn prefix(self) -> &'static str {
        match self {
            Alpha => "a",
            Beta => "b",
            Release => ""
        }
    }
}


/// A version struct tuple used to represent common versions from alpha.
///
/// Pre-release and release candidate are not yet representable with this.
///
/// The numbers are in the order: major version (always 1 for now),
/// minor version, patch version.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub struct Version(pub VersionType, pub u16, pub u16, pub u16);


impl Version {

    pub const RELEASE_1_3_2: Version = Version(Release, 1, 3, 2);
    pub const RELEASE_1_3_1: Version = Version(Release, 1, 3, 1);
    pub const RELEASE_1_3: Version   = Version(Release, 1, 3, 0);

    pub const RELEASE_1_2_5: Version = Version(Release, 1, 2, 5);
    pub const RELEASE_1_2_4: Version = Version(Release, 1, 2, 4);
    pub const RELEASE_1_2_3: Version = Version(Release, 1, 2, 3);
    pub const RELEASE_1_2_2: Version = Version(Release, 1, 2, 2);
    pub const RELEASE_1_2_1: Version = Version(Release, 1, 2, 1);
    pub const RELEASE_1_2: Version = Version(Release, 1, 2, 0);

    pub fn version_type(&self) -> VersionType { self.0 }
    pub fn major(&self) -> u16 { self.1 }
    pub fn minor(&self) -> u16 { self.2 }
    pub fn patch(&self) -> u16 { self.3 }

}


impl FromStr for Version {

    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        if s.is_empty() {
            return Err("Empty version".to_string());
        }

        let (version_type, s) = match &s[0..1] {
            "a" => (Alpha, &s[1..]),
            "b" => (Beta, &s[1..]),
            _ => (Release, s)
        };

        let mut v = Version(version_type, 0, 0, 0);

        for (i, part) in s.split(".").enumerate().take(3) {
            match part.parse::<u16>() {
                Ok(num) => match i {
                    0 => v.1 = num,
                    1 => v.2 = num,
                    2 => v.3 = num,
                    _ => {}
                }
                Err(e) => return Err(format!("Failed to parse part {} '{}': {}", i, part, e))
            }
        }

        Ok(v)

    }

}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.0.prefix())?;
        f.write_fmt(format_args!("{}.{}", self.1, self.2))?;
        if self.2 != 0 {
            f.write_fmt(format_args!(".{}", self.3))?;
        }
        Ok(())
    }
}
