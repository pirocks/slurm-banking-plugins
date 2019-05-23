/* 
 * Snippets API
 *
 * Test description
 *
 * OpenAPI spec version: v1
 * Contact: contact@snippets.local
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineResponse2003 {
  #[serde(rename = "count")]
  count: i32,
  #[serde(rename = "next")]
  next: Option<String>,
  #[serde(rename = "previous")]
  previous: Option<String>,
  #[serde(rename = "results")]
  results: Vec<::models::User>
}

impl InlineResponse2003 {
  pub fn new(count: i32, results: Vec<::models::User>) -> InlineResponse2003 {
    InlineResponse2003 {
      count: count,
      next: None,
      previous: None,
      results: results
    }
  }

  pub fn set_count(&mut self, count: i32) {
    self.count = count;
  }

  pub fn with_count(mut self, count: i32) -> InlineResponse2003 {
    self.count = count;
    self
  }

  pub fn count(&self) -> &i32 {
    &self.count
  }


  pub fn set_next(&mut self, next: String) {
    self.next = Some(next);
  }

  pub fn with_next(mut self, next: String) -> InlineResponse2003 {
    self.next = Some(next);
    self
  }

  pub fn next(&self) -> Option<&String> {
    self.next.as_ref()
  }

  pub fn reset_next(&mut self) {
    self.next = None;
  }

  pub fn set_previous(&mut self, previous: String) {
    self.previous = Some(previous);
  }

  pub fn with_previous(mut self, previous: String) -> InlineResponse2003 {
    self.previous = Some(previous);
    self
  }

  pub fn previous(&self) -> Option<&String> {
    self.previous.as_ref()
  }

  pub fn reset_previous(&mut self) {
    self.previous = None;
  }

  pub fn set_results(&mut self, results: Vec<::models::User>) {
    self.results = results;
  }

  pub fn with_results(mut self, results: Vec<::models::User>) -> InlineResponse2003 {
    self.results = results;
    self
  }

  pub fn results(&self) -> &Vec<::models::User> {
    &self.results
  }


}



