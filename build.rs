use postgres::{Client, NoTls};
use serde::Serialize;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::{error::Error, path::Path};

/// The header of the generated module
static HEADER: &'static str = "
//!
//! Lookup tools for EPSG Coordinate Reference System data.
//!
//! See https://epsg.org/terms-of-use.html.
//! # Example 
//! ```
//! use epsg::references::get_crs;
//! let wgs84 = get_crs(\"EPSG:4326\").unwrap();
//! assert_eq!(wgs84.coord_ref_sys_code, 4326);
//! assert_eq!(wgs84.coord_ref_sys_name, \"WGS 84\");
//! assert_eq!(wgs84.coord_ref_sys_kind, \"geographic 2D\");
//! assert_eq!(wgs84.coord_sys_code, 6422);
//! assert_eq!(wgs84.datum_code, 6326);
//! assert_eq!(wgs84.base_crs_code, 4979);
//! assert_eq!(wgs84.remarks, \"\");
//! assert_eq!(wgs84.data_source, \"EPSG\");
//! ```
use phf::{phf_map};
use crate::CRS;

static COORDINATE_REFS: phf::Map<&'static str, CRS> = phf_map! {
    ";

/// The map insertion call for each CRS
static TEMPLATE: &'static str = "
    \"{{data_source}}:{{sys_code}}\" => CRS {
        coord_ref_sys_code: {{sys_code}},
        coord_ref_sys_name: \"{{sys_name}}\",
        coord_ref_sys_kind: \"{{sys_kind}}\",
        coord_sys_code: {{coord_sys_code}},
        datum_code: {{datum_code}}, 
        base_crs_code: {{base_crs_code}},
        remarks: \"{{remarks}}\",
        information_source: \"{{information_source}}\",
        data_source: \"{{data_source}}\",
        revision_date: \"{{revision_date}}\",
        deprecated: {{deprecated}}, 
    },
";

/// The footer of the generated module
static FOOTER: &'static str = "
}; // close phf_map 

/// Find the specified CRS
pub fn get_crs(code: &str) -> Option<&CRS> {
    COORDINATE_REFS.get(code)
}

/// Search for the name for the given crs Authority:Code combination
///
/// e.g.
/// ```
/// use epsg::references::get_name;
/// assert_eq!(get_name(\"EPSG:4326\"), Some(\"WGS 84\"));
/// ```
pub fn get_name(crs: &str) -> Option<&'static str> {
    get_crs(&crs).map(|x| x.coord_ref_sys_name)
}

#[cfg(test)]
mod tests {
    use crate::references::{get_crs, get_name};

    #[test]
    fn test_lookup() {
        assert!(get_crs(\"EPSG:4326\").is_some());
        assert!(get_crs(\"blah\").is_none());
    }
}
    ";

static QUERY: &'static str = "
        SELECT coord_ref_sys_code, coord_ref_sys_name, coord_ref_sys_kind, coord_sys_code, datum_code, base_crs_code, remarks, information_source, data_source, revision_date, deprecated
        FROM epsg_coordinatereferencesystem; 
    ";

#[derive(Serialize)]
struct Record {
    sys_code: i32,
    sys_name: String,
    sys_kind: String,
    coord_sys_code: i32,
    datum_code: i32,
    base_crs_code: i32,
    remarks: String,
    information_source: String,
    data_source: String,
    revision_date: String,
    deprecated: i16,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=database/*.sql");
    if let Ok(pg_connection_str) = std::env::var("PG_STR") {
        // e.g. "host=localhost user=postgres password=postgres dbname=epsg"
        let mut client =
            Client::connect(&pg_connection_str, NoTls).expect("Failed to connect to postgres");

        let mut f = if Path::exists(Path::new("./src/references.rs")) {
            OpenOptions::new()
                .write(true)
                .truncate(true)
                .open("./src/references.rs")?
        } else {
            File::create("src/references.rs")?
        };

        let handlebars = handlebars::Handlebars::new();
        f.write_all(format!("{}", HEADER).as_bytes())
            .expect("Could not write header");
        for row in client.query(QUERY, &[]).expect("Could not query") {
            let sys_code: i32 = row.get(0);
            let sys_name: &str = row.get(1);
            let sys_kind: &str = row.get(2);
            let coord_sys_code: Option<i32> = row.get(3);
            let datum_code: Option<i32> = row.get(4);
            let base_crs_code: Option<i32> = row.get(5);
            let remarks: Option<&str> = row.get(6);
            let information_source: Option<&str> = row.get(7);
            let data_source: &str = row.get(8);
            let revision_date: time::Date = row.get(9);
            let deprecated: i16 = row.get(10);

            let crs = Record {
                sys_code,
                sys_name: sys_name.to_string(),
                sys_kind: sys_kind.to_string(),
                coord_sys_code: coord_sys_code.unwrap_or(-1),
                datum_code: datum_code.unwrap_or(-1),
                base_crs_code: base_crs_code.unwrap_or(-1),
                remarks: remarks.map(|x| x.to_string()).unwrap_or("".to_string()),
                information_source: information_source
                    .map(|x| x.to_string())
                    .unwrap_or("".to_string()),
                data_source: data_source.to_string(),
                revision_date: revision_date.to_string(),
                deprecated,
            };

            f.write_all(
                format!("{}", handlebars.render_template(&TEMPLATE, &crs).unwrap()).as_bytes(),
            )
            .expect("Could not write rendered template");
            f.flush().expect("Could not flush");
        }
        f.write_all(format!("{}", FOOTER).as_bytes())
            .expect("Could not write footer");
    }
    Ok(())
}
