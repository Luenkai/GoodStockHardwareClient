use serde::{Deserialize, Serialize};
use std::{sync::mpsc, time::Duration};

#[derive(Clone, Serialize, Deserialize)]
pub struct Product { pub sku:String, pub name:String, pub category:String, pub description:String, pub price:f64, pub cost:f64, pub stock:i32, pub max_stock:i32 }
#[derive(Clone, Serialize, Deserialize)]
pub struct Order { pub number:String, pub items:Vec<(String,i32,f64)>, pub total:f64, pub method:String, pub customer:String }
#[derive(Clone, Serialize, Deserialize)]
pub struct Account { pub email:String, pub password:String }
#[derive(Serialize, Deserialize)]
pub struct State { pub products:Vec<Product>, pub orders:Vec<Order>, pub accounts:Vec<Account>, pub next_order:u32, pub next_pos:u32 }
impl Default for State { fn default()->Self {Self{products:serde_json::from_str(include_str!("../data/catalog.json")).expect("embedded catalog"),orders:Vec::new(),accounts:Vec::new(),next_order:1,next_pos:1}} }
impl State { pub fn load()->(Self,String) { match std::fs::read_to_string("goodstock_state.json") { Ok(s)=>match serde_json::from_str(&s) {Ok(state)=>(state,"Local state loaded".into()),Err(e)=>(Self::default(),format!("Could not read state: {e}; existing file was not overwritten"))},Err(e) if e.kind()==std::io::ErrorKind::NotFound=>(Self::default(),"Ready".into()),Err(e)=>(Self::default(),format!("Could not read state: {e}; existing file was not overwritten"))} } pub fn save(&self)->Result<(),String> { let data=serde_json::to_vec_pretty(self).map_err(|e|e.to_string())?; std::fs::write("goodstock_state.json.tmp",data).and_then(|_|std::fs::rename("goodstock_state.json.tmp","goodstock_state.json")).map_err(|e|e.to_string()) } }
// The endpoint exposes availability, not inventory quantities. Preserve local stock and pricing.
pub fn sync() -> mpsc::Receiver<Result<Vec<serde_json::Value>,String>> { let (tx,rx)=mpsc::channel(); std::thread::spawn(move||{let result=(|| {let base=std::env::var("GOODSTOCK_API_URL").map_err(|_|"Set GOODSTOCK_API_URL to enable sync".to_string())?;let url=format!("{}/api/trpc/storefront.products",base.trim_end_matches('/'));let agent=ureq::AgentBuilder::new().timeout(Duration::from_secs(15)).build(); let value:serde_json::Value=agent.get(&url).call().map_err(|e|e.to_string())?.into_json().map_err(|e|format!("Invalid server response: {e}"))?;let data=&value["result"]["data"];data.as_array().or_else(||data["json"].as_array()).cloned().ok_or_else(||"Unexpected catalog response".to_string())})();let _=tx.send(result);});rx }
