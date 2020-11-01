//! Tools and data for coordinate reference systems from the [EPSG Geodetic Parameter Dataset](https://en.wikipedia.org/wiki/EPSG_Geodetic_Parameter_Dataset).
pub mod references;

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
