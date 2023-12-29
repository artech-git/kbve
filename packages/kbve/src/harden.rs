use tower_http::cors::CorsLayer;
use axum::{
	response::{ IntoResponse },
	http::{
		header,
		HeaderValue,
		StatusCode,
		Method,
		Uri,
	},
	Json,
};

use regex::Regex;
use once_cell::sync::Lazy;

use uuid::Uuid;
use num_bigint::{ BigUint };

use reqwest::Client;
use serde::{ Deserialize, Serialize };

use std::collections::HashMap;
use std::str::FromStr;

use crate::{wh::{ WizardResponse }, errors::error_types::ProcessResult};

pub static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(
  r"(?i)^[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}$"
).unwrap());

pub static GITHUB_USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(
    r"github\.com/([a-zA-Z0-9_-]+)"
).unwrap());

pub static INSTAGRAM_USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(
    r"(?:@|(?:www\.)?instagram\.com/)?(?:@)?([a-zA-Z0-9_](?:[a-zA-Z0-9_.]*[a-zA-Z0-9_])?)"
).unwrap());

pub static UNSPLASH_PHOTO_ID_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(
    r"photo-([a-zA-Z0-9]+-[a-zA-Z0-9]+)"
).unwrap());


pub fn validate_password(password: &str) -> Result<(), &str> {
	// Check if the password is long enough (e.g., at least 8 characters)
	if password.chars().count() < 8 {
		return Err("Password is too short");
	}

	// Check if the password is not too long (e.g., no more than 255 characters)
	if password.chars().count() > 255 {
		return Err("Password is too long");
	}

	// Check for a mix of uppercase and lowercase characters, numbers, and special characters
	let has_uppercase = password.chars().any(|c| c.is_uppercase());
	let has_lowercase = password.chars().any(|c| c.is_lowercase());
	let has_digit = password.chars().any(|c| c.is_digit(10));
	let has_special = password.chars().any(|c| !c.is_alphanumeric());

	if !has_uppercase || !has_lowercase || !has_digit || !has_special {
		return Err(
			"Password must include uppercase, lowercase, digits, and special characters"
		);
	}

	Ok(())
}

pub fn sanitize_email(email: &str) -> Result<String, &str> {
	let email = email.trim().to_lowercase();

	if email.chars().count() > 254 {
		return Err("Email is more than 254 characters");
	}

	if EMAIL_REGEX.is_match(&email) {
		Ok(email)
	} else {
		Err("Invalid email format")
	}
}

pub fn sanitize_username(username: &str) -> ProcessResult<String> {
	let sanitized: String = username
		.chars()
		.filter(|c| c.is_alphanumeric() && c.is_ascii())
		.collect();

	if sanitized.chars().count() < 6 {
		return Err("Username is too short");
	}

	if sanitized.chars().count() > 255 {
		return Err("Username is too long");
	}

	if sanitized != username {
		return Err("Username contains invalid characters".into());
	}

	if sanitized.is_empty() {
		return Err("Username cannot be empty");
	}

	Ok(sanitized)
}

pub fn sanitize_uuid(uuid_str: &str) -> ProcessResult<u64> {
	match uuid_str.parse::<u64>() {
		Ok(uuid) => {
			// You can add additional checks here if needed
			Ok(uuid)
		}
		Err(_) => Err("Invalid UUID format"),
	}
}

pub fn sanitize_input(input: &str) -> String {
	let mut sanitized: String = input
		.chars()
		.filter(|c| c.is_alphanumeric() && c.is_ascii())
		.collect();

	if sanitized.chars().count() > 255 {
		sanitized.truncate(255);
	}

	sanitized
}

pub fn sanitize_string_limit(input: &str) -> String {
	let mut sanitized: String = ammonia::clean(input);

	if sanitized.chars().count() > 255 {
		if let Some((idx, _)) = sanitized.char_indices().nth(255) {
			sanitized.truncate(idx);
		}
	}

	sanitized
}

pub fn sanitize_path(input: &str) -> String {
	let mut sanitized: String = input
		.chars()
		.filter(|c| (c.is_alphanumeric() || "/?@%$#".contains(*c)))
		.collect();

	if sanitized.chars().count() > 255 {
		sanitized.truncate(255);
	}

	sanitized
}

//  ?   [Regex] Extractions

pub fn extract_instagram_username(url: &str) -> Option<String> {
	INSTAGRAM_USERNAME_REGEX.captures(url)
		.and_then(|cap| {
			cap.get(1).map(|username| {
				let username = username.as_str();
				if
					username.contains("__") ||
					username.contains("._") ||
					username.contains("_.")
				{
					None
				} else {
					Some(username.to_string())
				}
			})
		})
		.flatten()
}

pub fn extract_github_username(url: &str) -> Option<String> {
	GITHUB_USERNAME_REGEX.captures(url).and_then(|cap| {
		cap.get(1).map(|username| username.as_str().to_string())
	})
}

pub fn extract_unsplash_photo_id(url: &str) -> Option<String> {
	UNSPLASH_PHOTO_ID_REGEX.captures(url).and_then(|cap| {
		cap.get(1).map(|match_| match_.as_str().to_string())
	})
}

//	? - Convert

pub fn uuid_to_biguint(uuid_str: &str) -> Result<BigUint, &'static str> {
	let uuid = Uuid::from_str(uuid_str).map_err(|_| "Invalid UUID format")?;
	let bytes = uuid.as_bytes();
	Ok(BigUint::from_bytes_be(bytes))
}


pub fn num_to_bigint(uuid_str: &str) -> Result<Uuid, &'static str> {
	let uuid = Uuid::from_str(uuid_str).map_err(|_| "Invalid UUID format")?;
	// let bytes = num_str.as_bytes();
	Ok(uuid)
	// Ok(BigUint::from_bytes_be(bytes))
	// Ok(BigUint::from_str(num_str).unwrap())
}


pub async fn fallback(uri: Uri) -> impl IntoResponse {
	let final_path = sanitize_path(uri.path());

	let response = WizardResponse {
		data: serde_json::json!({"status": "error"}),
		message: serde_json::json!({"path": final_path}),
	};

	(StatusCode::NOT_FOUND, Json(response))
}

pub fn cors_service() -> ProcessResult<CorsLayer> {

	let origins = [
			"https://herbmail.com",
			"https://kbve.com",
			"https://discord.sh",
			"http://localhost:3000",
			"https://kbve.itch.io"
		];

	let allowed_origins = {
		let mut allowed_origins = vec![]; 

		for header in origins.into_iter() { 
			let parsed_header = HeaderValue::from_str(header)?;
			allowed_origins.push(parsed_header);
		}
		allowed_origins
	};

	let allowed_methods = { 
		[Method::PUT, Method::GET, Method::DELETE, Method::POST]
	};

	let allowed_headers = { 
		[
			header::AUTHORIZATION,
			header::ACCEPT,
			header::CONTENT_TYPE,
			header::HeaderName::from_static("x-kbve-shieldwall"),
			header::HeaderName::from_static("x-kbve-api"),
		]
	};


	let cors_layer = CorsLayer::new()
		.allow_origin(allowed_origins)
		.allow_methods(allowed_methods)
		.allow_credentials(true)
		.allow_headers(allowed_headers);

	Ok(cors_layer)
}

#[derive(Serialize, Deserialize)]
struct CaptchaResponse {
	success: bool,
}

pub async fn verify_captcha(
	captcha_token: &str
) -> ProcessResult<bool> {

	let secret = match crate::wh::GLOBAL.get() {
		Some(global_map) =>
			match global_map.get("hcaptcha") {
				Some(value) => value.value().to_owned(),
				None => {
					return Err("missing_captcha".into());
				}
			}
		None => {
			return Err("invalid_global_map".into());
		}
	};

	let mut params = { 
		[
			("response", captcha_token).into(), 
			("secret", secret.as_str()).into()
		]
	};

	let res = (reqwest::Client::new())
		.post("https://api.hcaptcha.com/siteverify")
		.form(&params)
		.send()
		.await?;

	let captcha_response: CaptchaResponse = res.json().await?;

	Ok(captcha_response.success)
}



#[cfg(test)]
mod tests{

	use super::*;

	#[test]
	fn test_uuid_str() {
		let s = "739bcb2d312ec9c448c19450c827b";

		let res = num_to_bigint(s);
		println!("res: {:?}",res);
		assert!(true);
	}
}
