use std::ffi::OsStr;

pub trait Versioned {
    fn version(&self) -> Version;
}

pub trait AsStr {
    fn as_str(&self) -> &str;
}

macro_rules! as_os_str {
    ($t:ty) => {
        impl AsRef<OsStr> for $t {
            fn as_ref(&self) -> &OsStr {
                self.as_str().as_ref()
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Action {
    ExtractXmlFromPdf,
    A3Only,
    CombineXmlAndPdf,
    Ubl,
    Upgrade,
    Validate,
    XmlToHtml,
    XmlToPdf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Format {
    FacturX,
    Zugferd,
    OrderX,
    CrossIndustryDespatchAdvice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Version {
    V1,
    V2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProfileV1 {
    BASIC,
    COMFORT,
    EXTENDED,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProfileV2 {
    MINIMUM,
    BasicWl,
    BASIC,
    CIUS,
    EN16931,
    XRechnung,
    EXTENDED,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProfileCrossIndustryDespatchAdvice {
    // https://github.com/ZUGFeRD/mustangproject/blob/d2948c63acda7c40caa9f46d65ace8a54aff9eff/Mustang-CLI/src/main/java/org/mustangproject/commandline/Main.java#L690
    Pilot,
}

impl Versioned for ProfileV1 {
    fn version(&self) -> Version {
        Version::V1
    }
}

impl Versioned for ProfileV2 {
    fn version(&self) -> Version {
        Version::V2
    }
}

impl Versioned for ProfileCrossIndustryDespatchAdvice {
    fn version(&self) -> Version {
        // https://github.com/ZUGFeRD/mustangproject/blob/d2948c63acda7c40caa9f46d65ace8a54aff9eff/Mustang-CLI/src/main/java/org/mustangproject/commandline/Main.java#L690
        Version::V1
    }
}

impl Versioned for Config {
    fn version(&self) -> Version {
        match self {
            Self::ZugferdV1 { profile } => profile.version(),
            Self::FacturXOrZugferdV2 { profile } => profile.version(),
            Self::OrderX { profile } => profile.version(),
            Self::CrossIndustryDespatchAdvice { profile } => profile.version(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Config {
    ZugferdV1 {
        profile: ProfileV1,
    },
    FacturXOrZugferdV2 {
        profile: ProfileV2,
    },
    OrderX {
        profile: ProfileV1,
    },
    CrossIndustryDespatchAdvice {
        profile: ProfileCrossIndustryDespatchAdvice,
    },
}

impl Config {
    pub fn profile_as_str(&self) -> &str {
        match self {
            Self::ZugferdV1 { profile } => profile.as_str(),
            Self::FacturXOrZugferdV2 { profile } => profile.as_str(),
            Self::OrderX { profile } => profile.as_str(),
            Self::CrossIndustryDespatchAdvice { profile } => profile.as_str(),
        }
    }
}

as_os_str!(ProfileV2);
impl AsStr for ProfileV2 {
    fn as_str(&self) -> &str {
        match self {
            Self::MINIMUM => "M",
            Self::BasicWl => "W",
            Self::BASIC => "B",
            Self::CIUS => "C",
            Self::EN16931 => "E",
            Self::XRechnung => "X",
            Self::EXTENDED => "T",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Language {
    En,
    Fr,
    De,
}

as_os_str!(Language);
impl AsStr for Language {
    fn as_str(&self) -> &str {
        match self {
            Self::En => "en",
            Self::Fr => "fr",
            Self::De => "de",
        }
    }
}

as_os_str!(ProfileV1);
impl AsStr for ProfileV1 {
    fn as_str(&self) -> &str {
        match self {
            Self::BASIC => "B",
            Self::COMFORT => "C",
            Self::EXTENDED => "T",
        }
    }
}

as_os_str!(Version);
impl AsStr for Version {
    fn as_str(&self) -> &str {
        match self {
            Self::V1 => "1",
            Self::V2 => "2",
        }
    }
}

as_os_str!(ProfileCrossIndustryDespatchAdvice);
impl AsStr for ProfileCrossIndustryDespatchAdvice {
    fn as_str(&self) -> &str {
        match self {
            Self::Pilot => "P",
        }
    }
}

as_os_str!(Action);
impl AsStr for Action {
    fn as_str(&self) -> &str {
        match self {
            Self::ExtractXmlFromPdf => "extract",
            Self::A3Only => "a3only",
            Self::CombineXmlAndPdf => "combine",
            Self::Ubl => "ubl",
            Self::Upgrade => "upgrade",
            Self::Validate => "validate",
            Self::XmlToHtml => "visualize",
            Self::XmlToPdf => "pdf",
        }
    }
}

as_os_str!(Format);
impl AsStr for Format {
    fn as_str(&self) -> &str {
        match self {
            Self::FacturX => "fx",
            Self::Zugferd => "zf",
            Self::OrderX => "ox",
            Self::CrossIndustryDespatchAdvice => "da",
        }
    }
}
