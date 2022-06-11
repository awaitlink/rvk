use super::*;

/// <https://vk.com/dev/objects/stats_format>
#[derive(Deserialize, Clone, Debug)]
pub struct Stats {
    pub period_from: String,
    pub period_to: String,
    pub visitors: Visitors,
    pub reach: Reach,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Visitors {
    pub views: Integer,
    pub visitors: Integer,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Reach {
    pub reach: Integer,
    pub reach_subscribers: Integer,
    pub mobile_reach: Integer,
    pub sex: Vec<SpecificStats>,
    pub age: Vec<SpecificStats>,
    pub sex_age: Vec<SpecificStats>,
    pub cities: Vec<CitiesStats>,
    pub countries: Vec<CountriesStats>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SpecificStats {
    pub value: String,
    pub count: Integer,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CitiesStats {
    pub name: String,
    pub city_id: String,
    pub count: Integer,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CountriesStats {
    pub name: String,
    pub code: String,
    pub country_id: Integer,
    pub count: Integer,
}
