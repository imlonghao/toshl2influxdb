use rand;
use rand::Rng;
use reqwest;
use std::env;
use std::thread;
use std::time::Duration;

mod toshl;

fn main() {
    let token = env::var("TOKEN").expect("TOKEN is empty");
    let dburl = env::var("DBURL").expect("DBURL is empty");
    let dbuser = env::var("DBUSER").expect("DBUSER is empty");
    let dbpass = env::var("DBPASS").expect("DBPASS is empty");
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
        match write_point(money, &dburl, &dbuser, &dbpass) {
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

fn write_point(
    money: f64,
    dburl: &String,
    dbuser: &String,
    dbpass: &String,
) -> Result<bool, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    client
        .post(dburl)
        .basic_auth(dbuser, Some(dbpass))
        .body(format!("toshl value={}", money))
        .send()?;
    Ok(true)
}
