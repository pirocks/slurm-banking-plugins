/* 
 * myBRC REST API
 *
 * REST API for myBRC
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
  #[serde(rename = "userid")]
  userid: Option<i32>,
  #[serde(rename = "accounts")]
  accounts: Option<Vec<String>>,
  #[serde(rename = "username")]
  username: Option<String>,
  #[serde(rename = "usermetadata")]
  usermetadata: Option<String>,
  #[serde(rename = "email")]
  email: Option<String>,
  #[serde(rename = "ldapuid")]
  ldapuid: Option<i32>,
  #[serde(rename = "created")]
  created: Option<String>,
  #[serde(rename = "updated")]
  updated: Option<String>
}

impl User {
  pub fn new() -> User {
    User {
      userid: None,
      accounts: None,
      username: None,
      usermetadata: None,
      email: None,
      ldapuid: None,
      created: None,
      updated: None
    }
  }

  pub fn set_userid(&mut self, userid: i32) {
    self.userid = Some(userid);
  }

  pub fn with_userid(mut self, userid: i32) -> User {
    self.userid = Some(userid);
    self
  }

  pub fn userid(&self) -> Option<&i32> {
    self.userid.as_ref()
  }

  pub fn reset_userid(&mut self) {
    self.userid = None;
  }

  pub fn set_accounts(&mut self, accounts: Vec<String>) {
    self.accounts = Some(accounts);
  }

  pub fn with_accounts(mut self, accounts: Vec<String>) -> User {
    self.accounts = Some(accounts);
    self
  }

  pub fn accounts(&self) -> Option<&Vec<String>> {
    self.accounts.as_ref()
  }

  pub fn reset_accounts(&mut self) {
    self.accounts = None;
  }

  pub fn set_username(&mut self, username: String) {
    self.username = Some(username);
  }

  pub fn with_username(mut self, username: String) -> User {
    self.username = Some(username);
    self
  }

  pub fn username(&self) -> Option<&String> {
    self.username.as_ref()
  }

  pub fn reset_username(&mut self) {
    self.username = None;
  }

  pub fn set_usermetadata(&mut self, usermetadata: String) {
    self.usermetadata = Some(usermetadata);
  }

  pub fn with_usermetadata(mut self, usermetadata: String) -> User {
    self.usermetadata = Some(usermetadata);
    self
  }

  pub fn usermetadata(&self) -> Option<&String> {
    self.usermetadata.as_ref()
  }

  pub fn reset_usermetadata(&mut self) {
    self.usermetadata = None;
  }

  pub fn set_email(&mut self, email: String) {
    self.email = Some(email);
  }

  pub fn with_email(mut self, email: String) -> User {
    self.email = Some(email);
    self
  }

  pub fn email(&self) -> Option<&String> {
    self.email.as_ref()
  }

  pub fn reset_email(&mut self) {
    self.email = None;
  }

  pub fn set_ldapuid(&mut self, ldapuid: i32) {
    self.ldapuid = Some(ldapuid);
  }

  pub fn with_ldapuid(mut self, ldapuid: i32) -> User {
    self.ldapuid = Some(ldapuid);
    self
  }

  pub fn ldapuid(&self) -> Option<&i32> {
    self.ldapuid.as_ref()
  }

  pub fn reset_ldapuid(&mut self) {
    self.ldapuid = None;
  }

  pub fn set_created(&mut self, created: String) {
    self.created = Some(created);
  }

  pub fn with_created(mut self, created: String) -> User {
    self.created = Some(created);
    self
  }

  pub fn created(&self) -> Option<&String> {
    self.created.as_ref()
  }

  pub fn reset_created(&mut self) {
    self.created = None;
  }

  pub fn set_updated(&mut self, updated: String) {
    self.updated = Some(updated);
  }

  pub fn with_updated(mut self, updated: String) -> User {
    self.updated = Some(updated);
    self
  }

  pub fn updated(&self) -> Option<&String> {
    self.updated.as_ref()
  }

  pub fn reset_updated(&mut self) {
    self.updated = None;
  }

}



