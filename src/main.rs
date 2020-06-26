use rand;
use rand::Rng;
use reqwest;
use std::env;
use std::thread;
use std::time::Duration;

mod influxdb;
mod toshl;

fn main() {
    let (token, dburl, dbuser, dbpass) = parse_env();
    let db = influxdb::InfluxDB::new(dburl, dbuser, dbpass);
    let mut rng = rand::thread_rng();
    loop {
        let minute = rng.gen_range(1, 60);
        let money = match fetch_toshl(&token) {
            Ok(x) => x,
            Err(e) => {
                println!("fetch_toshl failed, {}", e);
                thread::sleep(Duration::from_secs(60 * 5));
                continue;
            }
        };
        println!("{}", money);
        match db.write_point(money) {
            Ok(x) => x,
            Err(e) => {
                println!("write_point failed, {}", e);
                thread::sleep(Duration::from_secs(60 * 5));
                continue;
            }
        };
        thread::sleep(Duration::from_secs(60 * minute))
    }
}

fn parse_env() -> (String, String, String, String) {
    let token = env::var("TOKEN").expect("TOKEN is empty");
    let dburl = env::var("DBURL").expect("DBURL is empty");
    let dbuser = env::var("DBUSER").expect("DBUSER is empty");
    let dbpass = env::var("DBPASS").expect("DBPASS is empty");
    (token, dburl, dbuser, dbpass)
}

fn fetch_toshl(token: &String) -> Result<f64, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get("https://api.toshl.com/accounts")
        .bearer_auth(token)
        .send()?;
    let resp_json = resp.json::<toshl::Toshl>()?;
    let mut balance: f64 = 0.0;
    for account in resp_json {
        balance += account.balance / account.currency.rate;
    }
    Ok(balance)
}
