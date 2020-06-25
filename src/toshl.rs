use serde_derive::Deserialize;

pub type Toshl = Vec<ToshlElement>;

#[derive(Deserialize)]
pub struct ToshlElement {
    pub balance: f64,
    pub currency: Currency,
}

#[derive(Deserialize)]
pub struct Currency {
    pub rate: f64,
}
