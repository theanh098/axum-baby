pub struct QueryBuider {
  query: String,
}

impl QueryBuider {
  pub fn new() -> Self {
    Self {
      query: String::new(),
    }
  }

  pub fn get_query(&self) -> String {
    self.query.to_owned()
  }

  pub fn r#where(&mut self, conditon: impl Into<String>) {
    self.query = format!("WHERE {}", conditon.into());
  }

  pub fn and_where(&mut self, conditon: impl Into<String>) {
    if self.query.is_empty() {
      self.r#where(conditon);
    } else {
      self
        .query
        .push_str(format!(" AND {}", conditon.into()).as_str());
    }
  }
}
