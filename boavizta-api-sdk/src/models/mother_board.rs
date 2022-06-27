/*
 * BOAVIZTAPI - DEMO
 *
 * # 🎯 Retrieving the impacts of digital elements This is a quick demo, to see full documentation [click here](https://doc.api.boavizta.org)  ## ➡️Server router  ### Server routers support the following impacts:  | Impact | 🔨 Manufacture | 🔌 Usage | |--------|----------------|----------| |   GWP  |        X       |     X    | |   ADP  |        X       |     X    | |   PE   |        X       |     X    | ## ➡️Cloud router  ### Cloud routers support the following impacts:  | Impact | 🔨 Manufacture | 🔌 Usage | |--------|----------------|----------| |   GWP  |        X       |     X    | |   ADP  |        X       |     X    | |   PE   |        X       |     X    | ## ➡️Component router  ### Component routers support the following impacts:  | Impact | 🔨 Manufacture | 🔌 Usage | |--------|----------------|----------| |   GWP  |        X       |          | |   ADP  |        X       |          | |   PE   |        X       |          |
 *
 * The version of the OpenAPI document: 0.1.2
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MotherBoard {
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<i32>,
}

impl MotherBoard {
    pub fn new() -> MotherBoard {
        MotherBoard { units: None }
    }
}
