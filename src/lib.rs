//! Tools and data for coordinate reference systems from the [EPSG Geodetic Parameter Dataset](https://en.wikipedia.org/wiki/EPSG_Geodetic_Parameter_Dataset).
pub mod references;
use crate::references::get_crs;
use std::convert::TryFrom;

/// A coordinate reference system
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRS {
    pub coord_ref_sys_code: i32,
    pub coord_ref_sys_name: &'static str,
    pub coord_ref_sys_kind: &'static str,
    pub coord_sys_code: i32,
    pub datum_code: i32,
    pub base_crs_code: i32,
    pub remarks: &'static str,
    pub information_source: &'static str,
    pub data_source: &'static str,
    pub revision_date: &'static str,
    pub deprecated: i16,
}

impl CRS {
    /// Is this CRS deprecated? Convenience function for `deprecated` field.
    pub fn deprecated(&self) -> bool { 
        self.deprecated == 1
    }

}

impl TryFrom<String> for CRS {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        get_crs(&value).map(|x| x.to_owned()).ok_or("No such CRS")
    }
}

#[cfg(test)]
mod test {
    use crate::references::get_crs;

    #[test]
    fn test_deprecated() {
        let crs = get_crs("EPSG:2155").unwrap();
        assert!(crs.deprecated());
    }
}
