pub struct InfluxDB {
    url: String,
    user: String,
    pass: String,
}

impl InfluxDB {
    pub fn new(url: String, user: String, pass: String) -> InfluxDB {
        InfluxDB {
            url: url,
            user: user,
            pass: pass,
        }
    }
    pub fn write_point(&self, money: f64) -> Result<bool, reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        client
            .post(&self.url)
            .basic_auth(&self.user, Some(&self.pass))
            .body(format!("toshl value={}", money))
            .send()?;
        Ok(true)
    }
}
