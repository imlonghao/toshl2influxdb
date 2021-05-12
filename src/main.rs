use rand;
use rand::Rng;
use reqwest;
use std::env;
use std::thread;
use std::time::Duration;
use vigil_reporter::Reporter;

mod influxdb;
mod toshl;

fn main() {
    let (
        token,
        dburl,
        dbuser,
        dbpass,
        vigil_host,
        vigil_secret,
        vigil_probe_id,
        vigil_node_id,
        vigil_replica_id,
    ) = parse_env();
    if vigil_host != "" {
        vigil_worker(
            &vigil_host,
            &vigil_secret,
            &vigil_probe_id,
            &vigil_node_id,
            &vigil_replica_id,
        );
    }
    let db = influxdb::InfluxDB::new(dburl, dbuser, dbpass);
    let mut rng = rand::thread_rng();
    loop {
        let minute = rng.gen_range(1..60);
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

fn parse_env() -> (
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
) {
    let token = env::var("TOKEN").expect("TOKEN is empty");
    let dburl = env::var("DBURL").expect("DBURL is empty");
    let dbuser = env::var("DBUSER").expect("DBUSER is empty");
    let dbpass = env::var("DBPASS").expect("DBPASS is empty");
    let vigil_host = env::var("VIGIL_HOST").unwrap_or("".to_string());
    let vigil_secret = env::var("VIGIL_SECRET").unwrap_or("".to_string());
    let vigil_probe_id = env::var("VIGIL_PROBE_ID").unwrap_or("".to_string());
    let vigil_node_id = env::var("VIGIL_NODE_ID").unwrap_or("".to_string());
    let vigil_replica_id = env::var("VIGIL_REPLICA_ID").unwrap_or("".to_string());
    (
        token,
        dburl,
        dbuser,
        dbpass,
        vigil_host,
        vigil_secret,
        vigil_probe_id,
        vigil_node_id,
        vigil_replica_id,
    )
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

fn vigil_worker(
    vigil_host: &String,
    vigil_secret: &String,
    vigil_probe_id: &String,
    vigil_node_id: &String,
    vigil_replica_id: &String,
) {
    let reporter = Reporter::new(vigil_host, vigil_secret)
        .probe_id(vigil_probe_id) // Probe ID containing the parent Node for Replica
        .node_id(vigil_node_id) // Node ID containing Replica
        .replica_id(vigil_replica_id) // Unique Replica ID for instance (ie. your IP on the LAN)
        .build();
    reporter.run().expect("failed to start vigil reporter");
}
