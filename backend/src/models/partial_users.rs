
use actix_web::FromRequest;
use fake::Fake;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tiberius::ToSql;
use tiberius::{Client, Config};
use tokio_util::{compat::Compat, compat::TokioAsyncWriteCompatExt};

#[derive(Debug, derive_new::new, Deserialize, Serialize)]
pub struct partial_user {
    id: i32,
    address: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

impl partial_user {
pub fn to_alter_query(&self) -> (String, Vec<Arc<dyn ToSql>>) {
        let mut query = "UPDATE Users SET ".to_string();
        let mut params: Vec<Arc<dyn ToSql>> = vec![];
        let mut param_index = 1;

        if let Some(ref address) = self.address {
            query += &format!("Address=@P{}, ", param_index);
            params.push(Arc::new(address.clone()));
            param_index += 1;
        }
        if let Some(ref first_name) = self.first_name {
            query += &format!("FirstName=@P{}, ", param_index);
            params.push(Arc::new(first_name.clone()));
            param_index += 1;
        }
        if let Some(ref last_name) = self.last_name {
            query += &format!("LastName=@P{}, ", param_index);
            params.push(Arc::new(last_name.clone()));
            param_index += 1;
        }
        if let Some(ref username) = self.username {
            query += &format!("Username=@P{}, ", param_index);
            params.push(Arc::new(username.clone()));
            param_index += 1;
        }
        if let Some(ref password) = self.password {
            query += &format!("Password=@P{}, ", param_index);
            params.push(Arc::new(password.clone()));
            param_index += 1;
        }
        // Trim the trailing comma and space
        query = query.trim_end_matches(", ").to_string();
        query += &format!(" WHERE ID=@P{};", param_index);
        params.push(Arc::new(self.id));
        (query, params)
    }
}
