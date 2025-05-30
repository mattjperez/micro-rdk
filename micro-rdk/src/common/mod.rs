//! Structs, traits, and utils to develop [component](https://docs.viam.com/components/)
//! drivers.
//!
//! # Components
//! - [actuator]
//! - [base]
//! - [board]
//! - [camera]
//! - [encoder]
//! - [motor]
//! - [movement_sensor]
//! - [sensor]
//! - [servo]
//!
//! # Utils
//! - [grpc]
//! - [grpc_client]
//! - [i2c]
//! - [webrtc]
//! - [conn]
//!
//!
//! General Purpose Drivers
//! - [adxl345]
//! - [gpio_motor]
//! - [ina]
//! - [mpu6050]

pub mod actuator;
#[cfg(feature = "builtin-components")]
pub mod adxl345;
pub mod analog;
pub mod app_client;
pub mod base;
pub mod board;
pub mod button;
#[cfg(feature = "camera")]
pub mod camera;
pub mod config;
pub mod config_monitor;
pub mod credentials_storage;
pub mod digital_interrupt;
pub mod encoder;
pub mod exec;
pub mod generic;
#[cfg(feature = "builtin-components")]
pub mod gpio_motor;
#[cfg(feature = "builtin-components")]
pub mod gpio_servo;
pub mod grpc;
pub mod grpc_client;
pub mod i2c;
#[cfg(feature = "builtin-components")]
pub mod ina;
pub mod log;
pub mod math_utils;
pub mod motor;
pub mod movement_sensor;
#[cfg(feature = "builtin-components")]
pub mod mpu6050;
#[cfg(feature = "ota")]
pub mod ota;
pub mod power_sensor;
pub mod registry;
pub mod restart_monitor;
pub mod robot;
pub mod runtime;
pub mod sensor;
pub mod servo;
pub mod status;
pub mod switch;
pub mod system;
#[cfg(feature = "builtin-components")]
pub mod wheeled_base;
pub mod webrtc {
    pub mod api;
    pub mod candidates;
    pub mod certificate;
    pub mod dtls;
    pub mod exec;
    pub mod grpc;
    pub mod ice;
    pub mod io;
    pub mod sctp;
    pub mod signaling_server;
    pub mod udp_mux;
}
pub mod conn {
    pub mod errors;
    pub mod mdns;
    pub mod network;
    pub mod server;
    pub mod viam;
}
#[cfg(feature = "data")]
pub mod data_collector;
#[cfg(feature = "data")]
pub mod data_manager;
#[cfg(feature = "data")]
pub mod data_store;

pub mod provisioning;
