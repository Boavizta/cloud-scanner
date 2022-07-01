use isocountry::CountryCode;

/// Returns 3 letters ISO coutnry code code corresponding to an aws region
pub fn get_iso_country(aws_region: &str) -> &'static str {
    let cc = get_country_from_aws_region(aws_region);
    cc.alpha3()
}

/// Converts aws region into country code
pub fn get_country_from_aws_region(aws_region: &str) -> CountryCode {
    let cc: CountryCode = match aws_region {
        "eu-central-1" => CountryCode::DEU,
        "eu-east-1" => CountryCode::IRL,
        "eu-north-1" => CountryCode::SWE,
        "eu-south-1" => CountryCode::ITA,
        "eu-west-1" => CountryCode::IRL,
        "eu-west-2" => CountryCode::GBR,
        "eu-west-3" => CountryCode::FRA,
        _ => {
            error!("Unable to match aws region to country code, defaulting to FRA !");
            CountryCode::FRA
        }
    };
    cc
}

#[test]
fn test_get_country_code_from_region() {
    let region = "eu-west-3";
    let cc = get_country_from_aws_region(region);
    assert_eq!("FRA", cc.alpha3());
    //assert_eq!("IRL", get_country_from_aws_region("eu-west-1").alpha3());
}

#[test]
fn test_get_current_iso_region() {
    let aws_region = "eu-west-1";
    let country_code = get_iso_country(aws_region);
    assert_eq!("IRL", country_code);
    let aws_region = "eu-west-2";
    let country_code = get_iso_country(aws_region);
    assert_eq!("FR", country_code);
}
