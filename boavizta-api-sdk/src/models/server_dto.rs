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
pub struct ServerDto {
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<Box<crate::models::ModelServer>>,
    #[serde(rename = "configuration", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Box<crate::models::ConfigurationServer>>,
    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<Box<crate::models::UsageServer>>,
    #[serde(rename = "add_method", skip_serializing_if = "Option::is_none")]
    pub add_method: Option<String>,
    #[serde(rename = "add_date", skip_serializing_if = "Option::is_none")]
    pub add_date: Option<String>,
}

impl ServerDto {
    pub fn new() -> ServerDto {
        ServerDto {
            model: None,
            configuration: None,
            usage: None,
            add_method: None,
            add_date: None,
        }
    }
}
