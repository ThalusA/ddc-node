use ddc::{Ddc, DdcHost, DdcTable};
use ddc_hi::Display;
use mccs_db::ValueType;
use napi::bindgen_prelude::*;
use napi::{Error, Status};
use napi_derive::napi;
use std::collections::HashMap;

#[napi(js_name = "Display")]
pub struct JsDisplay {
  pub index: u32,
  pub backend: String,
  pub edid_data: Option<Uint8Array>,
  pub version: Option<String>,
  pub mccs_version: Option<String>,
  pub display_id: String,
  pub serial: Option<u32>,
  pub serial_number: Option<String>,
  pub model_id: Option<u16>,
  pub model_name: Option<String>,
  pub manufacturer_id: Option<String>,
  pub manufacture_year: Option<u8>,
  pub manufacture_week: Option<u8>,
  pub capabilities: Option<String>,
  display: Display,
}

#[napi]
pub enum VcpValueType {
  Continuous,
  NonContinuous,
  Table,
}

#[napi(object)]
pub struct Continuous {
  pub current_value: u16,
  pub maximum_value: u16,
  #[napi(ts_type = "VcpValueType.Continuous")]
  pub r#type: VcpValueType,
}

#[napi(object)]
pub struct NonContinuous {
  pub current_value: u16,
  pub current_value_representation: Option<String>,
  pub possible_values: HashMap<String, Option<String>>,
  #[napi(ts_type = "VcpValueType.NonContinuous")]
  pub r#type: VcpValueType,
}

#[napi(object)]
pub struct Table {
  pub current_data: Vec<u8>,
  #[napi(ts_type = "VcpValueType.Table")]
  pub r#type: VcpValueType,
}

#[napi]
impl JsDisplay {
  #[napi(constructor)]
  pub fn new(index: u32) -> Result<Self> {
    Display::enumerate()
      .into_iter()
      .nth(index as usize)
      .map(|display| JsDisplay::from_display(index, display))
      .ok_or(Error::new(
        Status::InvalidArg,
        format!("Out of bound: There is no display at index {}", index),
      ))
  }

  pub fn from_display(index: u32, mut display: Display) -> Self {
    JsDisplay {
      index,
      backend: display.info.backend.to_string().clone(),
      edid_data: display
        .info
        .edid_data
        .clone()
        .map(|edid_data| Uint8Array::new(edid_data)),
      version: display
        .info
        .version
        .map(|(major, minor)| format!("{}.{}", major, minor))
        .clone(),
      mccs_version: display
        .info
        .mccs_version
        .map(|mcc_version| mcc_version.to_string())
        .clone(),
      display_id: display.info.id.clone(),
      serial: display.info.serial,
      serial_number: display.info.serial_number.clone(),
      model_id: display.info.model_id,
      model_name: display.info.model_name.clone(),
      manufacturer_id: display.info.manufacturer_id.clone(),
      manufacture_year: display.info.manufacture_year,
      manufacture_week: display.info.manufacture_week,
      capabilities: display
        .handle
        .capabilities_string()
        .map_or(None, |capabilities_string| {
          std::str::from_utf8(capabilities_string.as_slice()).map_or(None, |capabilities_string| {
            Some(capabilities_string.to_string())
          })
        }),
      display,
    }
  }

  #[napi]
  pub async fn get_vcp_feature(
    &mut self,
    feature_code: u8,
  ) -> Result<Either3<Continuous, NonContinuous, Table>> {
    let feature = self.display.info.mccs_database.get(feature_code);
    let handle = &mut self.display.handle;
    if let Some(feature) = feature {
      match feature.ty {
        ValueType::Unknown => {
          let vcp_feature_value = handle
            .get_vcp_feature(feature_code)
            .map_err(|error| Error::new(Status::GenericFailure, error.to_string()))?;
          let current_value = vcp_feature_value.value();
          let maximum_value = vcp_feature_value.maximum();
          handle.sleep();
          Ok(Either3::A(Continuous {
            current_value,
            maximum_value,
            r#type: VcpValueType::Continuous,
          }))
        }
        ValueType::Continuous {
          interpretation: _interpretation,
        } => {
          let vcp_feature_value = handle
            .get_vcp_feature(feature_code)
            .map_err(|error| Error::new(Status::GenericFailure, error.to_string()))?;
          let current_value = vcp_feature_value.value();
          let maximum_value = vcp_feature_value.maximum();
          handle.sleep();
          Ok(Either3::A(Continuous {
            current_value,
            maximum_value,
            r#type: VcpValueType::Continuous,
          }))
        }
        ValueType::NonContinuous {
          ref values,
          interpretation: _interpretation,
        } => {
          let vcp_feature_value = handle
            .get_vcp_feature(feature_code)
            .map_err(|error| Error::new(Status::GenericFailure, error.to_string()))?;
          let mut current_value = (vcp_feature_value.value(), None);
          let possible_values = HashMap::from_iter(
            values
              .clone()
              .into_iter()
              .map(|(value, value_representation)| (value.to_string(), value_representation)),
          );
          if let Some(&Some(ref name)) = values.get(&(vcp_feature_value.value() as u8)) {
            current_value.1 = Some(name.clone());
          }
          handle.sleep();
          Ok(Either3::B(NonContinuous {
            current_value: current_value.0,
            current_value_representation: current_value.1,
            possible_values,
            r#type: VcpValueType::NonContinuous,
          }))
        }
        ValueType::Table {
          interpretation: _interpretation,
        } => {
          let vcp_feature_value = handle
            .table_read(feature_code)
            .map_err(|error| Error::new(Status::GenericFailure, error.to_string()))?;
          handle.sleep();
          Ok(Either3::C(Table {
            current_data: vcp_feature_value,
            r#type: VcpValueType::Table,
          }))
        }
      }
    } else {
      match handle.table_read(feature_code) {
        Ok(vcp_feature_value) => {
          handle.sleep();
          Ok(Either3::C(Table {
            current_data: vcp_feature_value,
            r#type: VcpValueType::Table,
          }))
        }
        Err(_) => {
          let vcp_feature_value = handle
            .get_vcp_feature(feature_code)
            .map_err(|error| Error::new(Status::GenericFailure, error.to_string()))?;
          let current_value = vcp_feature_value.value();
          let maximum_value = vcp_feature_value.maximum();
          handle.sleep();
          Ok(Either3::A(Continuous {
            current_value,
            maximum_value,
            r#type: VcpValueType::Continuous,
          }))
        }
      }
    }
  }

  #[napi]
  pub async fn set_vcp_feature(
    &mut self,
    feature_code: u8,
    value_or_offset: u16,
    bytes: Option<Vec<u8>>,
  ) -> Result<()> {
    if let Some(bytes) = bytes {
      let result = self
        .display
        .handle
        .table_write(feature_code, value_or_offset, &bytes)
        .map_err(|error| Error::new(Status::GenericFailure, error.to_string()))?;
      self.display.handle.sleep();
      Ok(result)
    } else {
      let result = self
        .display
        .handle
        .set_vcp_feature(feature_code, value_or_offset)
        .map_err(|error| Error::new(Status::GenericFailure, error.to_string()))?;
      self.display.handle.sleep();
      Ok(result)
    }
  }
}
