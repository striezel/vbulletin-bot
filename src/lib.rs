/*
 -------------------------------------------------------------------------------
    This file is part of the vBulletin moderation bot.
    Copyright (C) 2021  Dirk Stolle
    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.
    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.
    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
 -------------------------------------------------------------------------------
*/

/// Internal data for API requests
struct ApiData
{
  /// version of the API, usually 1 or higher
  version: u32,

  /// access token for API requests
  access_token: String,

  /// client id assigned to the client
  client_id: String,

  /// secret that is used in signature generation
  secret: String
}

/// This class handles basic API requests.
pub struct VBulletinApi
{
  /// internal data
  data: ApiData,

  /// full base URL of forum, e. g. https://domain.example.com/vb4/
  base_url: String,

  /// user for HTTP BasicAuth, if any
  basic_auth_user: String,

  /// password for HTTP BasicAuth, if any
  basic_auth_pass: String
}

impl VBulletinApi
{
  /* Generates a new VBulletinApi instance.
   *
   * @param  base_url   URL of the vBulletin installation, e. g. "https://forum.example.com/vbulletin/"
   * @param  basic_auth_user  user name for HTTP BasicAuth (leave empty, if BasicAuth is not used)
   * @param  basic_auth_pass  password for HTTP BasicAuth (leave empty, if BasicAuth is not used)
   * @return Returns a Result containing the new instance, if initialization succeeded.
   *         Returns an error message otherwise.
   */
  pub fn new(base_url: &str, basic_auth_user: &str, basic_auth_pass: &str) -> Result<VBulletinApi, String>
  {
    let api_data = VBulletinApi::api_init(base_url, basic_auth_user, basic_auth_pass)?;
    Ok(VBulletinApi{
      data: api_data,
      base_url: base_url.to_string(),
      basic_auth_user: basic_auth_user.to_string(),
      basic_auth_pass: basic_auth_pass.to_string()
    })
  }

  /* Initializes the API.
   *
   * @param  base_url   URL of the vBulletin installation, e. g. "https://forum.example.com/vbulletin/"
   * @param  basic_auth_user  user name for HTTP BasicAuth (leave empty, if BasicAuth is not used)
   * @param  basic_auth_pass  password for HTTP BasicAuth (leave empty, if BasicAuth is not used)
   * @return Returns a Result containing the ApiData, if initialization succeeded.
   *         Returns an error message otherwise.
   */
  fn api_init(base_url: &str, basic_auth_user: &str, basic_auth_pass: &str) -> Result<ApiData, String>
  {
    use reqwest::StatusCode;
    use serde_json::Value;

    let version = match option_env!("CARGO_PKG_VERSION")
    {
      Some(v) => format!("corona, version {}", v),
      None => String::from("1.0")
    };
    let name = option_env!("CARGO_PKG_NAME").unwrap_or("vbulletin-api");
    let unique_id = "555logs";
    let url = format!("{}/api.php?api_m=api_init&clientname={}&clientversion={}&platformname={}&platformversion={}&uniqueid={}",
                      base_url, name, version, name, version, unique_id);

    let client = reqwest::blocking::Client::new();
    let mut builder = client.get(&url);
    if !basic_auth_user.is_empty() && !basic_auth_pass.is_empty()
    {
      builder = builder.basic_auth(basic_auth_user, Some(basic_auth_pass));
    }

    let mut response = match builder.send()
    {
      Ok(responded) => responded,
      Err(e) => return Err(format!("API request failed: {}", e))
    };
    // Response is not guaranteed to be UTF-8, could be ISO-8859-1 or similar.
    let mut body: Vec<u8> = vec![];
    if let Err(e) = response.copy_to(&mut body)
    {
      return Err(format!("Failed to read API response: {}", e));
    }
    let body = String::from_utf8_lossy(&body);

    if response.status() != StatusCode::OK
    {
      return Err(format!("HTTP request failed with unexpected status code: {}\n\
                        Headers:\n{:#?}\n\
                        Body:\n{}", response.status(), response.headers(), body));
    }
    let json: Value = match serde_json::from_str(&body)
    {
      Ok(v) => v,
      Err(e) => return Err(format!("Failed to deserialize JSON from API: {}", e))
    };

    let version = match json.get("apiversion")
    {
      Some(value) => match value.as_u64()
      {
        Some(int) => int as u32,
        None => return Err("Response contains 'apiversion', but it's not an integer!".to_string())
      }
      None => return Err(format!("Response contains no 'apiversion'!\nResponse is:\n{}\n", body))
    };
    let client_id = match json.get("apiclientid")
    {
      Some(value) => match value.as_str()
      {
        Some(str) => str,
        None => return Err(format!("Response contains 'apiclientid', but it's not a string!\nResponse is:\n{}\n", body))
      }
      None => return Err("Response contains no 'apiclientid'!".to_string())
    };
    let access_token = match json.get("apiaccesstoken")
    {
      Some(value) => match value.as_str()
      {
        Some(str) => str,
        None => return Err("Response contains 'apiaccesstoken', but it's not a string!".to_string())
      }
      None => return Err("Response contains no 'apiaccesstoken'!".to_string())
    };
    let secret = match json.get("secret")
    {
      Some(value) => match value.as_str()
      {
        Some(str) => str,
        None => return Err("Response contains 'secret', but it's not a string!".to_string())
      }
      None => return Err("Response contains no 'secret'!".to_string())
    };

    Ok(ApiData{ version, access_token: access_token.to_string(),
                client_id: client_id.to_string(), secret: secret.to_string() })
  }

  /* Attempts to perform a login.
   *
   * @param  username  user name of the vB user account
   * @param  password  password of the vB user account
   * @return Returns Ok(()), if initialization succeeded.
   *         Returns an error message otherwise.
   */
  pub fn login(&self, username: &str, password: &str) -> Result<(), String>
  {
    use reqwest::blocking::multipart;

    let url = format!("{}/login.php?do=login&s=", self.base_url);
    let client = reqwest::blocking::Client::new();
    let mut builder = client.post(&url);
    if !self.basic_auth_user.is_empty() && !self.basic_auth_pass.is_empty()
    {
      builder = builder.basic_auth(&self.basic_auth_user, Some(&self.basic_auth_pass));
    }

    let form = multipart::Form::new()
      .text("do", "login")
      .text("securitytoken", "guest")
      .text("s", "")
      .text("cookieuser", "1")
      .text("vb_login_username", username.to_string())
      .text("vb_login_password", password.to_string());

    let mut response = match builder.multipart(form).send()
    {
      Ok(responded) => responded,
      Err(e) =>
      {
        return Err(format!("Login request failed: {}", e));
      }
    };
    // Response is not guaranteed to be UTF-8, could be ISO-8859-1 or similar.
    let mut body: Vec<u8> = vec![];
    if let Err(e) = response.copy_to(&mut body)
    {
      return Err(format!("Failed to read API response: {}", e));
    }
    let body = String::from_utf8_lossy(&body);

    if response.status() != reqwest::StatusCode::OK
    {
      return Err(format!("HTTP request failed with unexpected status code: {}\n\
                        Headers:\n{:#?}\n\
                        Body:\n{}", response.status(), response.headers(), body));
    }

    if body.contains("exec_refresh")
    {
      Ok(())
    }
    else
    {
      Err("Login has failed.".to_string())
    }
  }
}