use crate::win32::*;
use std::fmt;

#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
/// MSDN: [D3D_FEATURE_LEVEL](https://docs.microsoft.com/en-us/windows/desktop/api/d3dcommon/ne-d3dcommon-d3d_feature_level)
pub struct FeatureLevel(pub(crate) D3D_FEATURE_LEVEL);

impl FeatureLevel {
    pub const _9_1  : FeatureLevel = FeatureLevel(D3D_FEATURE_LEVEL_9_1);
    pub const _9_2  : FeatureLevel = FeatureLevel(D3D_FEATURE_LEVEL_9_2);
    pub const _9_3  : FeatureLevel = FeatureLevel(D3D_FEATURE_LEVEL_9_3);
    pub const _10_0 : FeatureLevel = FeatureLevel(D3D_FEATURE_LEVEL_10_0);
    pub const _10_1 : FeatureLevel = FeatureLevel(D3D_FEATURE_LEVEL_10_1);
    pub const _11_0 : FeatureLevel = FeatureLevel(D3D_FEATURE_LEVEL_11_0);
    pub const _11_1 : FeatureLevel = FeatureLevel(D3D_FEATURE_LEVEL_11_1);
    pub const _12_0 : FeatureLevel = FeatureLevel(D3D_FEATURE_LEVEL_12_0);
    pub const _12_1 : FeatureLevel = FeatureLevel(D3D_FEATURE_LEVEL_12_1);
}

impl fmt::Debug for FeatureLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FeatureLevel::_9_1  => "FeatureLevel 9.1".fmt(f),
            FeatureLevel::_9_2  => "FeatureLevel 9.2".fmt(f),
            FeatureLevel::_9_3  => "FeatureLevel 9.3".fmt(f),
            FeatureLevel::_10_0 => "FeatureLevel 10.0".fmt(f),
            FeatureLevel::_10_1 => "FeatureLevel 10.1".fmt(f),
            FeatureLevel::_11_0 => "FeatureLevel 11.0".fmt(f),
            FeatureLevel::_11_1 => "FeatureLevel 11.1".fmt(f),
            FeatureLevel::_12_0 => "FeatureLevel 12.0".fmt(f),
            FeatureLevel::_12_1 => "FeatureLevel 12.1".fmt(f),
            _                   => write!(f, "FeatureLevel ??.? ({})", self.0),
        }
    }
}
