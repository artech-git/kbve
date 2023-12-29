// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]
use diesel::prelude::*;
use serde::{ Serialize, Deserialize};



use chrono::NaiveDateTime;
#[derive(Queryable, Serialize, Deserialize, Debug, Identifiable)]
#[diesel(primary_key(ulid))]
pub struct Apikey {
    pub permissions: String,
    pub keyhash: String,
    pub label: String,
    pub ulid: Vec<u8>,
    pub userid: Vec<u8>,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Identifiable)]
#[diesel(primary_key(ulid))]
pub struct Appwrite {
    pub appwrite_endpoint: String,
    pub appwrite_projectid: String,
    pub appwrite_api_key: String,
    pub version: String,
    pub created_at: NaiveDateTime,
    pub ulid: Vec<u8>,
    pub userid: Vec<u8>,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Identifiable)]
#[diesel(primary_key(ulid))]
pub struct Auth {
    pub email: String,
    pub hash: String,
    pub salt: String,
    pub password_reset_token: String,
    pub password_reset_expiry: NaiveDateTime,
    pub verification_token: String,
    pub verification_expiry: NaiveDateTime,
    pub status: i32,
    pub last_login_at: NaiveDateTime,
    pub failed_login_attempts: i32,
    pub lockout_until: NaiveDateTime,
    pub two_factor_secret: String,
    pub recovery_codes: String,
    pub ulid: Vec<u8>,
    pub userid: Vec<u8>,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Identifiable)]
pub struct Global {
    pub id: u64,
    pub key: String,
    pub value: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Identifiable)]
#[diesel(primary_key(ulid))]
pub struct N8n {
    pub webhook: String,
    pub permissions: String,
    pub keyhash: String,
    pub label: String,
    pub ulid: Vec<u8>,
    pub userid: Vec<u8>,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Identifiable)]
#[diesel(primary_key(ulid))]
pub struct Profile {
    pub name: String,
    pub bio: String,
    pub unsplash: String,
    pub github: String,
    pub instagram: String,
    pub discord: String,
    pub ulid: Vec<u8>,
    pub userid: Vec<u8>,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Identifiable)]
#[diesel(primary_key(ulid))]
pub struct Setting {
    pub key: String,
    pub value: String,
    pub ulid: Vec<u8>,
    pub userid: Vec<u8>,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Identifiable)]
#[diesel(primary_key(ulid))]
pub struct User {
    pub username: String,
    pub role: i32,
    pub reputation: i32,
    pub exp: i32,
    pub created_at: NaiveDateTime,
    pub ulid: Vec<u8>,
}

