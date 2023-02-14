use serde:: {
    Deserialize, Serialize
};


//DASHBOARD PAGE
// 1. Data untuk Dashboard (ping dan searches)
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DashboardData {
    pub request_amount: Option<u32>,
    pub ping : Option<u32>,
}