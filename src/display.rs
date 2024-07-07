use ddc::{Ddc, DdcHost, DdcTable};
use mccs_db::ValueType;
use napi::bindgen_prelude::*;
use napi::JsUndefined;
use napi_derive::napi;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Display(pub ddc_hi::Display);

#[cfg(target_os = "windows")]
unsafe impl Send for Display {}

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
  display: Arc<Mutex<Display>>,
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

pub struct AsyncGetVcp {
  display: Arc<Mutex<Display>>,
  feature_code: u8,
}

#[napi]
impl Task for AsyncGetVcp {
  type Output = Either3<Continuous, NonContinuous, Table>;
  type JsValue = Either3<Continuous, NonContinuous, Table>;

  fn compute(&mut self) -> Result<Self::Output> {
    let mut display = self.display.lock().unwrap();
    if let Some(feature) = display.0.info.mccs_database.get(self.feature_code).cloned() {
      match feature.ty {
        ValueType::Unknown => {
          let vcp_feature_value = display
            .0
            .handle
            .get_vcp_feature(self.feature_code)
            .map_err(|error| Error::new(Status::GenericFailure, error.to_string()))?;
          let current_value = vcp_feature_value.value();
          let maximum_value = vcp_feature_value.maximum();
          display.0.handle.sleep();
          Ok(Either3::A(Continuous {
            current_value,
            maximum_value,
            r#type: VcpValueType::Continuous,
          }))
        }
        ValueType::Continuous {
          interpretation: _interpretation,
        } => {
          let vcp_feature_value = display
            .0
            .handle
            .get_vcp_feature(self.feature_code)
            .map_err(|error| Error::new(Status::GenericFailure, error.to_string()))?;
          let current_value = vcp_feature_value.value();
          let maximum_value = vcp_feature_value.maximum();
          display.0.handle.sleep();
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
          let vcp_feature_value = display
            .0
            .handle
            .get_vcp_feature(self.feature_code)
            .map_err(|error| Error::new(Status::GenericFailure, error.to_string()))?;
          let mut current_value = (vcp_feature_value.value(), None);
          let possible_values = HashMap::from_iter(
            values
              .clone()
              .into_iter()
              .map(|(value, value_representation)| (value.to_string(), value_representation)),
          );
          if let Some(Some(name)) = values.get(&(vcp_feature_value.value() as u8)) {
            current_value.1 = Some(name.clone());
          }
          display.0.handle.sleep();
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
          let vcp_feature_value = display
            .0
            .handle
            .table_read(self.feature_code)
            .map_err(|error| Error::new(Status::GenericFailure, error.to_string()))?;
          display.0.handle.sleep();
          Ok(Either3::C(Table {
            current_data: vcp_feature_value,
            r#type: VcpValueType::Table,
          }))
        }
      }
    } else {
      match display.0.handle.table_read(self.feature_code) {
        Ok(vcp_feature_value) => {
          display.0.handle.sleep();
          Ok(Either3::C(Table {
            current_data: vcp_feature_value,
            r#type: VcpValueType::Table,
          }))
        }
        Err(_) => {
          let vcp_feature_value = display
            .0
            .handle
            .get_vcp_feature(self.feature_code)
            .map_err(|error| Error::new(Status::GenericFailure, error.to_string()))?;
          let current_value = vcp_feature_value.value();
          let maximum_value = vcp_feature_value.maximum();
          display.0.handle.sleep();
          Ok(Either3::A(Continuous {
            current_value,
            maximum_value,
            r#type: VcpValueType::Continuous,
          }))
        }
      }
    }
  }

  fn resolve(&mut self, _: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

pub struct AsyncSetVcp {
  display: Arc<Mutex<Display>>,
  feature_code: u8,
  value_or_offset: u16,
  bytes: Option<Vec<u8>>,
}

#[napi]
impl Task for AsyncSetVcp {
  type Output = ();
  type JsValue = JsUndefined;

  fn compute(&mut self) -> Result<Self::Output> {
    let mut display = self.display.lock().unwrap();
    if let Some(bytes) = self.bytes.clone() {
      display
        .0
        .handle
        .table_write(self.feature_code, self.value_or_offset, &bytes)
        .map_err(|error| Error::new(Status::GenericFailure, error.to_string()))?;
      display.0.handle.sleep();
      Ok(())
    } else {
      display
        .0
        .handle
        .set_vcp_feature(self.feature_code, self.value_or_offset)
        .map_err(|error| Error::new(Status::GenericFailure, error.to_string()))?;
      display.0.handle.sleep();
      Ok(())
    }
  }

  fn resolve(&mut self, env: Env, _: Self::Output) -> Result<Self::JsValue> {
    env.get_undefined()
  }
}

#[napi]
impl JsDisplay {
  #[napi(constructor)]
  pub fn new(index: u32) -> Result<Self> {
    ddc_hi::Display::enumerate()
      .into_iter()
      .nth(index as usize)
      .map(|display| JsDisplay::from_display(index, display))
      .ok_or_else(|| {
        Error::new(
          Status::InvalidArg,
          format!("Out of bound: There is no display at index {}", index),
        )
      })
  }

  pub fn from_display(index: u32, display: ddc_hi::Display) -> Self {
    let mut js_display = JsDisplay {
      index,
      backend: display.info.backend.to_string(),
      edid_data: display.info.edid_data.clone().map(Uint8Array::new),
      version: display
        .info
        .version
        .map(|(major, minor)| format!("{}.{}", major, minor)),
      mccs_version: display
        .info
        .mccs_version
        .map(|mcc_version| mcc_version.to_string()),
      display_id: display.info.id.clone(),
      serial: display.info.serial,
      serial_number: display.info.serial_number.clone(),
      model_id: display.info.model_id,
      model_name: display.info.model_name.clone(),
      manufacturer_id: display.info.manufacturer_id.clone(),
      manufacture_year: display.info.manufacture_year,
      manufacture_week: display.info.manufacture_week,
      capabilities: None,
      display: Arc::new(Mutex::new(Display(display))),
    };
    js_display.update_capabilities();
    js_display
  }

  #[napi]
  pub fn get_vcp_feature(&mut self, feature_code: u8) -> AsyncTask<AsyncGetVcp> {
    AsyncTask::new(AsyncGetVcp {
      display: self.display.clone(),
      feature_code,
    })
  }

  #[napi]
  pub fn set_vcp_feature(
    &mut self,
    feature_code: u8,
    value_or_offset: u16,
    bytes: Option<Vec<u8>>,
  ) -> AsyncTask<AsyncSetVcp> {
    AsyncTask::new(AsyncSetVcp {
      display: self.display.clone(),
      feature_code,
      value_or_offset,
      bytes,
    })
  }

  #[napi]
  pub fn update_capabilities(&mut self) -> Option<String> {
    let mut display = self.display.lock().unwrap();
    display
      .0
      .handle
      .capabilities_string()
      .map_or(None, |capabilities_string| {
        std::str::from_utf8(capabilities_string.as_slice()).map_or(None, |capabilities_string| {
          Some(capabilities_string.to_string())
        })
      })
  }
}
