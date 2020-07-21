use postgres::{Client, NoTls};
use handlebars::Handlebars;
use serde::{Serialize};
//use serde_json::json;
//

#[derive(Serialize)]
struct CRS {
    sys_code: i32,
    sys_name: String,
    sys_kind: String,
    coord_sys_code: Option<i32>,
    datum_code: Option<i32>,
    remarks: Option<String>,
    information_source: Option<String>,
    data_source: String,
    revision_date: String,
    deprecated: i16,
}

fn main() {
    let template: String = String::from("
        CRS {
            sys_code: {{sys_code}},
            sys_name: {{sys_name}}.to_string(),
            sys_kind: &str,
            coord_sys_code: Option<i32>,
            datum_code: Option<i32>,
            remarks: Option<&str>,
            information_source: Option<&str>,
            data_source: &str,
            revision_date: chrono::NaiveDate,
            deprecated: i16,
        }
    ");

    let mut client = Client::connect("host=localhost user=postgres password=postgres dbname=epsg", NoTls).expect("Failed to connect to postgres");
    let mut handlebars = Handlebars::new();
    let query = "
        SELECT coord_ref_sys_code, coord_ref_sys_name, coord_ref_sys_kind, coord_sys_code, datum_code, remarks, information_source, data_source, revision_date, deprecated
        FROM epsg_coordinatereferencesystem; 
    ";
    let mut data: Vec<CRS> = Vec::new();
    for row in client.query(query, &[]).expect("Could not query") {
	let sys_code: i32 = row.get(0);
	let sys_name: &str = row.get(1);
	let sys_kind: &str = row.get(2);
        let coord_sys_code: Option<i32> = row.get(3);
        let datum_code: Option<i32> = row.get(4);
        let remarks: Option<&str> = row.get(5);
        let information_source: Option<&str> = row.get(6);
        let data_source: &str = row.get(7);
        let revision_date: time::Date = row.get(8);
        let deprecated: i16 = row.get(9);

        let crs = CRS {
            sys_code: sys_code,
            sys_name: sys_name.to_string(),
            sys_kind: sys_kind.to_string(),
            coord_sys_code: coord_sys_code,
            datum_code: datum_code,
            remarks: remarks.map(|x| x.to_string()),
            information_source: information_source.map(|x| x.to_string()),
            data_source: data_source.to_string(),
            revision_date: revision_date.to_string(),
            deprecated: deprecated,
        };
        print!("{:?}", handlebars.render_template(&template, &crs).unwrap());
    }
}
