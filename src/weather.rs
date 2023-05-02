use serde::{Deserialize, Serialize};

pub type Root = Vec<Root2>;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root2 {
    pub publishing_office: String,
    pub report_datetime: String,
    pub time_series: Vec<TimeSery>,
    pub temp_average: Option<TempAverage>,
    pub precip_average: Option<PrecipAverage>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeSery {
    pub time_defines: Vec<String>,
    pub areas: Vec<Area>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Area {
    pub area: Area2,
    #[serde(default)]
    pub weather_codes: Vec<String>,
    pub weathers: Option<Vec<String>>,
    pub winds: Option<Vec<String>>,
    #[serde(default)]
    pub pops: Vec<String>,
    pub waves: Option<Vec<String>>,
    pub temps: Option<Vec<String>>,
    pub reliabilities: Option<Vec<String>>,
    pub temps_min: Option<Vec<String>>,
    pub temps_min_upper: Option<Vec<String>>,
    pub temps_min_lower: Option<Vec<String>>,
    pub temps_max: Option<Vec<String>>,
    pub temps_max_upper: Option<Vec<String>>,
    pub temps_max_lower: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Area2 {
    pub name: String,
    pub code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TempAverage {
    pub areas: Vec<Area3>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Area3 {
    pub area: Area4,
    pub min: String,
    pub max: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Area4 {
    pub name: String,
    pub code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrecipAverage {
    pub areas: Vec<Area5>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Area5 {
    pub area: Area6,
    pub min: String,
    pub max: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Area6 {
    pub name: String,
    pub code: String,
}
