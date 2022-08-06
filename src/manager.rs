use crate::display::JsDisplay;
use ddc_hi::{Backend, Display, Query};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::str::FromStr;

#[napi]
pub enum JsQueryType {
  Backend,
  Id,
  ManufacturerId,
  ModelName,
  SerialNumber,
}

#[napi(object, js_name = "Query")]
#[derive(Clone)]
pub struct JsQuery {
  pub query_type: JsQueryType,
  pub query_value: String,
}

#[napi(js_name = "DisplayManager")]
pub struct JsDisplayManager {
  queries: Vec<Query>,
}

impl TryFrom<JsQuery> for Query {
  type Error = Error;
  fn try_from(query: JsQuery) -> Result<Self> {
    match query.query_type {
      JsQueryType::Backend => Ok(Query::Backend(
        Backend::from_str(query.query_value.as_str()).map_err(|_| {
          Error::new(
            Status::InvalidArg,
            format!("{} is not a valid backend type", query.query_value),
          )
        })?,
      )),
      JsQueryType::Id => Ok(Query::Id(query.query_value)),
      JsQueryType::ManufacturerId => Ok(Query::ManufacturerId(query.query_value)),
      JsQueryType::ModelName => Ok(Query::ModelName(query.query_value)),
      JsQueryType::SerialNumber => Ok(Query::SerialNumber(query.query_value)),
    }
  }
}

impl TryFrom<Query> for JsQuery {
  type Error = Error;
  fn try_from(query: Query) -> Result<Self> {
    match query {
      Query::Backend(backend) => Ok(JsQuery {
        query_type: JsQueryType::Backend,
        query_value: backend.to_string(),
      }),
      Query::Id(id) => Ok(JsQuery {
        query_type: JsQueryType::Id,
        query_value: id,
      }),
      Query::ManufacturerId(manufacturer_id) => Ok(JsQuery {
        query_type: JsQueryType::ManufacturerId,
        query_value: manufacturer_id,
      }),
      Query::ModelName(model_name) => Ok(JsQuery {
        query_type: JsQueryType::ModelName,
        query_value: model_name,
      }),
      Query::SerialNumber(serial_number) => Ok(JsQuery {
        query_type: JsQueryType::SerialNumber,
        query_value: serial_number,
      }),
      _ => Err(Error::new(
        Status::InvalidArg,
        "Can't convert this query into JavaScript enum".to_owned(),
      )),
    }
  }
}

// Temporary fix: Query doesn't implement Copy and Clone trait
impl TryFrom<&Query> for JsQuery {
  type Error = Error;
  fn try_from(query: &Query) -> Result<Self> {
    match query {
      Query::Backend(backend) => Ok(JsQuery {
        query_type: JsQueryType::Backend,
        query_value: backend.to_string(),
      }),
      Query::Id(id) => Ok(JsQuery {
        query_type: JsQueryType::Id,
        query_value: id.to_owned(),
      }),
      Query::ManufacturerId(manufacturer_id) => Ok(JsQuery {
        query_type: JsQueryType::ManufacturerId,
        query_value: manufacturer_id.to_owned(),
      }),
      Query::ModelName(model_name) => Ok(JsQuery {
        query_type: JsQueryType::ModelName,
        query_value: model_name.to_owned(),
      }),
      Query::SerialNumber(serial_number) => Ok(JsQuery {
        query_type: JsQueryType::SerialNumber,
        query_value: serial_number.to_owned(),
      }),
      _ => Err(Error::new(
        Status::InvalidArg,
        "Can't convert this query into JavaScript enum".to_owned(),
      )),
    }
  }
}

#[napi]
impl JsDisplayManager {
  #[napi(constructor)]
  pub fn new(queries: Option<Either<JsQuery, Vec<JsQuery>>>) -> Result<Self> {
    match queries {
      Some(Either::A(query)) => Ok(JsDisplayManager {
        queries: vec![query.try_into()?],
      }),
      Some(Either::B(queries)) => Ok(JsDisplayManager {
        queries: queries
          .into_iter()
          .map(Query::try_from)
          .collect::<std::result::Result<Vec<Query>, Error>>()?,
      }),
      None => Ok(JsDisplayManager { queries: vec![] }),
    }
  }

  #[napi]
  pub fn get_by_index(index: u32) -> Result<JsDisplay> {
    JsDisplay::new(index)
  }

  #[napi(getter, js_name = "queries")]
  pub fn get_queries(&self) -> Result<Vec<JsQuery>> {
    self
      .queries
      .iter()
      .map(JsQuery::try_from)
      .collect::<std::result::Result<Vec<JsQuery>, Error>>()
  }

  #[napi(setter, js_name = "queries")]
  pub fn set_queries(&mut self, queries: Vec<JsQuery>) -> Result<()> {
    self.queries = queries
      .into_iter()
      .map(Query::try_from)
      .collect::<std::result::Result<Vec<Query>, Error>>()?;
    Ok(())
  }

  #[napi]
  pub fn add_queries(&mut self, queries: Option<Either<JsQuery, Vec<JsQuery>>>) -> Result<()> {
    self.queries.append(&mut match queries {
      Some(Either::A(query)) => vec![query.try_into()?],
      Some(Either::B(queries)) => queries
        .into_iter()
        .map(Query::try_from)
        .collect::<std::result::Result<Vec<Query>, Error>>()?,
      None => vec![],
    });
    Ok(())
  }

  #[napi]
  pub fn collect(&self) -> Vec<JsDisplay> {
    Display::enumerate()
      .into_iter()
      .enumerate()
      .map(|(index, display)| JsDisplay::from_display(index as u32, display))
      .collect()
  }

  #[napi]
  pub fn list(&self) -> Vec<JsDisplay> {
    self.collect()
  }
}
