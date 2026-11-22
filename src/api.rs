use argon2::{password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString}, Argon2};
use serde::{Deserialize, Serialize};
use std::{sync::mpsc, time::Duration};

#[derive(Clone, Serialize, Deserialize)]
pub struct Product {
    pub sku: String, pub name: String, pub category: String, pub description: String,
    pub price: f64, pub cost: f64, pub stock: i32, pub max_stock: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_id: Option<String>,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Order {
    pub number: String, pub items: Vec<(String, i32, f64)>, pub total: f64,
    pub method: String, pub customer: String,
    #[serde(default)] pub delivery_address: String,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Account {
    pub email: String,
    // Accept older state files so plaintext passwords can be migrated on load.
    #[serde(alias = "password")]
    pub password_hash: String,
}
#[derive(Serialize, Deserialize)]
pub struct State { pub products: Vec<Product>, pub orders: Vec<Order>, pub accounts: Vec<Account>, pub next_order: u32, pub next_pos: u32 }

pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut rand_core::OsRng);
    Argon2::default().hash_password(password.as_bytes(), &salt).map(|h| h.to_string()).map_err(|e| e.to_string())
}
pub fn verify_password(password: &str, hash: &str) -> bool {
    PasswordHash::new(hash).ok().is_some_and(|parsed| Argon2::default().verify_password(password.as_bytes(), &parsed).is_ok())
}
impl Default for State {
    fn default() -> Self { Self { products: serde_json::from_str(include_str!("../data/catalog.json")).expect("embedded catalog"), orders: Vec::new(), accounts: Vec::new(), next_order: 1, next_pos: 1 } }
}
impl State {
    pub fn load() -> (Self, String) {
        match std::fs::read_to_string("goodstock_state.json") {
            Ok(s) => match serde_json::from_str::<Self>(&s) {
                Ok(mut state) => {
                    let mut migrated = false;
                    for account in &mut state.accounts {
                        if !account.password_hash.starts_with("$argon2") {
                            match hash_password(&account.password_hash) {
                                Ok(hash) => { account.password_hash = hash; migrated = true; },
                                Err(e) => return (state, format!("Could not read state: password migration failed: {e}; existing file was not overwritten")),
                            }
                        }
                    }
                    if migrated {
                        if let Err(e) = state.save() { return (state, format!("Could not read state: password migration could not be saved: {e}; existing file was not overwritten")); }
                        (state, "Local accounts migrated to password hashes".into())
                    } else { (state, "Local state loaded".into()) }
                },
                Err(e) => (Self::default(), format!("Could not read state: {e}; existing file was not overwritten")),
            },
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => (Self::default(), "Ready".into()),
            Err(e) => (Self::default(), format!("Could not read state: {e}; existing file was not overwritten")),
        }
    }
    pub fn save(&self) -> Result<(), String> {
        let data = serde_json::to_vec_pretty(self).map_err(|e| e.to_string())?;
        let mut file = tempfile::NamedTempFile::new_in(".").map_err(|e| e.to_string())?;
        use std::io::Write;
        file.write_all(&data).and_then(|_| file.flush()).map_err(|e| e.to_string())?;
        file.persist("goodstock_state.json").map_err(|e| e.to_string())?;
        Ok(())
    }
    // Match by remote ID, unique SKU or unique title. New products start with zero local stock;
    // the public endpoint exposes availability but not a trustworthy inventory quantity.
    pub fn merge_catalog(&mut self, items: &[serde_json::Value]) -> (usize, usize, usize) {
        let (mut updated, mut added, mut skipped) = (0, 0, 0);
        for item in items {
            let Some(id) = item["id"].as_str().filter(|s| !s.is_empty()) else { skipped += 1; continue };
            let Some(name) = item["title"].as_str().filter(|s| !s.trim().is_empty()) else { skipped += 1; continue };
            let sku = item.pointer("/variants/nodes/0/sku").and_then(|s| s.as_str()).filter(|s| !s.trim().is_empty());
            let exact = self.products.iter().position(|p| p.remote_id.as_deref() == Some(id));
            let unique_match = |matches: Vec<usize>| if matches.len() == 1 { Some(matches[0]) } else { None };
            let index = exact.or_else(|| sku.and_then(|s| unique_match(self.products.iter().enumerate().filter(|(_, p)| p.remote_id.is_none() && p.sku.eq_ignore_ascii_case(s)).map(|(i, _)| i).collect())))
                .or_else(|| unique_match(self.products.iter().enumerate().filter(|(_, p)| p.remote_id.is_none() && p.name == name).map(|(i, _)| i).collect()));
            if let Some(i) = index {
                let p = &mut self.products[i];
                p.remote_id = Some(id.to_owned()); p.name = name.to_owned();
                if let Some(desc) = item["description"].as_str() { p.description = desc.to_owned(); }
                if let Some(category) = item["productType"].as_str() { p.category = category.to_owned(); }
                updated += 1;
            } else {
                let price = item.pointer("/priceRange/minVariantPrice/amount").and_then(|v| v.as_str().and_then(|s| s.parse::<f64>().ok()).or_else(|| v.as_f64())).filter(|v| v.is_finite() && *v >= 0.0).unwrap_or(0.0);
                let mut n = 1;
                let chosen_sku = if let Some(s) = sku.filter(|s| !self.products.iter().any(|p| p.sku.eq_ignore_ascii_case(s))) { s.to_owned() } else { loop { let candidate = format!("SYNC-{n:04}"); if !self.products.iter().any(|p| p.sku.eq_ignore_ascii_case(&candidate)) { break candidate } n += 1; } };
                self.products.push(Product { sku: chosen_sku, name: name.to_owned(), category: item["productType"].as_str().unwrap_or("Hardware").to_owned(), description: item["description"].as_str().unwrap_or("").to_owned(), price, cost: 0.0, stock: 0, max_stock: 40, remote_id: Some(id.to_owned()) });
                added += 1;
            }
        }
        (updated, added, skipped)
    }
}
// Read-only catalog synchronization; local orders and inventory are not sent to the server.
pub fn sync() -> mpsc::Receiver<Result<Vec<serde_json::Value>, String>> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let result = (|| {
            let base = std::env::var("GOODSTOCK_API_URL").map_err(|_| "Set GOODSTOCK_API_URL to enable sync".to_string())?;
            let url = format!("{}/api/trpc/storefront.products", base.trim_end_matches('/'));
            let agent = ureq::AgentBuilder::new().timeout(Duration::from_secs(15)).build();
            let response = agent.get(&url).call().map_err(|e| e.to_string())?;
            let value: serde_json::Value = serde_json::from_reader(response.into_reader()).map_err(|e| format!("Invalid server response: {e}"))?;
            let data = &value["result"]["data"];
            data.as_array().or_else(|| data["json"].as_array()).cloned().ok_or_else(|| "Unexpected catalog response".to_string())
        })();
        let _ = tx.send(result);
    });
    rx
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn passwords_are_salted_and_verified() {
        let first = hash_password("correct horse").unwrap(); let second = hash_password("correct horse").unwrap();
        assert_ne!(first, second); assert!(verify_password("correct horse", &first)); assert!(!verify_password("wrong", &first));
    }
    #[test] fn merge_is_idempotent_and_preserves_local_inventory() {
        let mut s = State::default(); let name = s.products[0].name.clone(); let stock = s.products[0].stock; let price = s.products[0].price;
        let products = vec![serde_json::json!({"id":"one","title":name,"description":"Updated","productType":"Fasteners"}), serde_json::json!({"id":"two","title":"New tool","priceRange":{"minVariantPrice":{"amount":"120"}}})];
        assert_eq!(s.merge_catalog(&products), (1, 1, 0)); assert_eq!(s.products[0].stock, stock); assert_eq!(s.products[0].price, price); assert_eq!(s.products.last().unwrap().stock, 0);
        assert_eq!(s.merge_catalog(&products), (2, 0, 0)); assert_eq!(s.products.len(), 13);
    }
    #[test] fn older_orders_and_accounts_deserialize() {
        let account: Account = serde_json::from_str(r#"{"email":"a","password":"legacy"}"#).unwrap(); assert_eq!(account.password_hash, "legacy");
        let order: Order = serde_json::from_str(r#"{"number":"GS-ORD-0001","items":[],"total":0,"method":"Counter Pickup","customer":""}"#).unwrap(); assert!(order.delivery_address.is_empty());
    }
}
