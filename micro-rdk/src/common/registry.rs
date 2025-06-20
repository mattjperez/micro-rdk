#![allow(dead_code)]
use std::collections::HashMap as Map;
use thiserror::Error;

use super::{
    base::{BaseError, BaseType},
    board::{BoardError, BoardType},
    button::{ButtonError, ButtonType},
    config::ConfigType,
    encoder::{EncoderError, EncoderType},
    generic::{GenericComponentType, GenericError},
    motor::{MotorError, MotorType},
    movement_sensor::MovementSensorType,
    power_sensor::PowerSensorType,
    robot::Resource,
    sensor::{SensorError, SensorType},
    servo::{ServoError, ServoType},
    switch::{SwitchError, SwitchType},
};

#[cfg(feature = "camera")]
use super::camera::{CameraError, CameraType};
use crate::proto::common::v1::ResourceName;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum RegistryError {
    #[error("RegistryError : Model '{0}' not found")]
    ModelNotFound(String),
    #[error("RegistryError : model '{0}' already exists")]
    ModelAlreadyRegistered(String),
    #[error("RegistryError: model '{0}' dependency getter already registered")]
    ModelDependencyFuncRegistered(String),
    #[error("RegistryError: dependencies unsupported for component type '{0}'")]
    ComponentTypeNotInDependencies(String),
    #[error("RegistryError: model '{0}' not found in dependencies under component type '{1}'")]
    ModelNotFoundInDependencies(String, String),
}

pub fn get_board_from_dependencies(deps: Vec<Dependency>) -> Option<BoardType> {
    for Dependency(_, dep) in deps {
        match dep {
            Resource::Board(b) => return Some(b.clone()),
            _ => continue,
        }
    }
    None
}

// ResourceKey is an identifier for a component to be registered to a robot. The
// first element is a string representing the component type (arm, motor, etc.)
// and the second element is its name.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ResourceKey(pub String, pub String);

impl ResourceKey {
    pub fn new(model: impl Into<String>, name: impl Into<String>) -> Self {
        Self(model.into(), name.into())
    }
}

impl TryFrom<ResourceName> for ResourceKey {
    type Error = RegistryError;
    fn try_from(value: ResourceName) -> Result<Self, Self::Error> {
        let comp_type: &str = &value.subtype;
        let comp_name = match comp_type {
            "button" => crate::common::button::COMPONENT_NAME,
            "motor" => crate::common::motor::COMPONENT_NAME,
            "sensor" => crate::common::sensor::COMPONENT_NAME,
            #[cfg(feature = "camera")]
            "camera" => crate::common::camera::COMPONENT_NAME,
            "movement_sensor" => crate::common::movement_sensor::COMPONENT_NAME,
            "encoder" => crate::common::encoder::COMPONENT_NAME,
            "base" => crate::common::base::COMPONENT_NAME,
            "servo" => crate::common::servo::COMPONENT_NAME,
            "switch" => crate::common::switch::COMPONENT_NAME,
            "power_sensor" => crate::common::power_sensor::COMPONENT_NAME,
            "generic" => crate::common::generic::COMPONENT_NAME,
            _ => {
                return Err(RegistryError::ModelNotFound(comp_type.into()));
            }
        };
        Ok(Self(comp_name.to_string(), value.name))
    }
}

pub struct Dependency(pub ResourceKey, pub Resource);

/// Fn that returns a `BoardType`, `Arc<Mutex<dyn Board>>`
type BoardConstructor = dyn Fn(ConfigType) -> Result<BoardType, BoardError>;

type ButtonConstructor = dyn Fn(ConfigType, Vec<Dependency>) -> Result<ButtonType, ButtonError>;

/// Fn that returns a `MotorType`, `Arc<Mutex<dyn Motor>>`
type MotorConstructor = dyn Fn(ConfigType, Vec<Dependency>) -> Result<MotorType, MotorError>;

/// Fn that returns a `SensorType`, `Arc<Mutex<dyn Sensor>>`
type SensorConstructor = dyn Fn(ConfigType, Vec<Dependency>) -> Result<SensorType, SensorError>;

/// Fn that returns a `MovementSensorType`, `Arc<Mutex<dyn MovementSensor>>`
type MovementSensorConstructor =
    dyn Fn(ConfigType, Vec<Dependency>) -> Result<MovementSensorType, SensorError>;

/// Fn that returns an `EncoderType`, `Arc<Mutex<dyn Encoder>>`
type EncoderConstructor = dyn Fn(ConfigType, Vec<Dependency>) -> Result<EncoderType, EncoderError>;

/// Fn that returns an `BaseType`, `Arc<Mutex<dyn Base>>`
type BaseConstructor = dyn Fn(ConfigType, Vec<Dependency>) -> Result<BaseType, BaseError>;

/// Fn that returns a `CameraType`, `Arc<Mutex<dyn Camera>>`
#[cfg(feature = "camera")]
type CameraConstructor = dyn Fn(ConfigType, Vec<Dependency>) -> Result<CameraType, CameraError>;

/// Fn that returns a `ServoType`, `Arc<Mutex<dyn Servo>>`
type ServoConstructor = dyn Fn(ConfigType, Vec<Dependency>) -> Result<ServoType, ServoError>;

type SwitchConstructor = dyn Fn(ConfigType, Vec<Dependency>) -> Result<SwitchType, SwitchError>;

/// Fn that returns a `PowerSensorType`, `Arc<Mutex<dyn PowerSensor>>`
type PowerSensorConstructor =
    dyn Fn(ConfigType, Vec<Dependency>) -> Result<PowerSensorType, SensorError>;

/// Fn that returns a `GenericComponentType`, `Arc<Mutex<dyn GenericComponentType>>`
type GenericComponentConstructor =
    dyn Fn(ConfigType, Vec<Dependency>) -> Result<GenericComponentType, GenericError>;

type DependenciesFromConfig = dyn Fn(ConfigType) -> Vec<ResourceKey>;

#[derive(Clone)]
pub struct ComponentRegistry {
    motors: Map<String, &'static MotorConstructor>,
    board: Map<String, &'static BoardConstructor>,
    buttons: Map<String, &'static ButtonConstructor>,
    #[cfg(feature = "camera")]
    camera: Map<String, &'static CameraConstructor>,
    sensor: Map<String, &'static SensorConstructor>,
    movement_sensors: Map<String, &'static MovementSensorConstructor>,
    encoders: Map<String, &'static EncoderConstructor>,
    bases: Map<String, &'static BaseConstructor>,
    servos: Map<String, &'static ServoConstructor>,
    switches: Map<String, &'static SwitchConstructor>,
    power_sensors: Map<String, &'static PowerSensorConstructor>,
    generic_components: Map<String, &'static GenericComponentConstructor>,
    dependencies: Map<String, Map<String, &'static DependenciesFromConfig>>,
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        let mut r = Self::new();
        crate::common::board::register_models(&mut r);
        #[cfg(feature = "builtin-components")]
        {
            crate::common::button::register_models(&mut r);
            crate::common::encoder::register_models(&mut r);
            crate::common::motor::register_models(&mut r);
            crate::common::gpio_motor::register_models(&mut r);
            crate::common::gpio_servo::register_models(&mut r);
            crate::common::sensor::register_models(&mut r);
            crate::common::servo::register_models(&mut r);
            crate::common::switch::register_models(&mut r);
            crate::common::movement_sensor::register_models(&mut r);
            crate::common::mpu6050::register_models(&mut r);
            crate::common::adxl345::register_models(&mut r);
            crate::common::generic::register_models(&mut r);
            crate::common::ina::register_models(&mut r);
            crate::common::wheeled_base::register_models(&mut r);
            #[cfg(feature = "camera")]
            crate::common::camera::register_models(&mut r);
        }
        #[cfg(feature = "esp32")]
        {
            crate::esp32::board::register_models(&mut r);
            #[cfg(feature = "builtin-components")]
            {
                crate::esp32::encoder::register_models(&mut r);
                crate::esp32::hcsr04::register_models(&mut r);
                crate::esp32::single_encoder::register_models(&mut r);
                crate::esp32::coredump::register_models(&mut r);
            }
        }
        r
    }
}

impl ComponentRegistry {
    pub fn new() -> Self {
        let mut dependency_func_map = Map::new();
        dependency_func_map.insert(crate::common::motor::COMPONENT_NAME.into(), Map::new());
        dependency_func_map.insert(
            crate::common::movement_sensor::COMPONENT_NAME.into(),
            Map::new(),
        );
        dependency_func_map.insert(crate::common::encoder::COMPONENT_NAME.into(), Map::new());
        dependency_func_map.insert(crate::common::sensor::COMPONENT_NAME.into(), Map::new());
        dependency_func_map.insert(crate::common::base::COMPONENT_NAME.into(), Map::new());
        #[cfg(feature = "camera")]
        dependency_func_map.insert(crate::common::camera::COMPONENT_NAME.into(), Map::new());
        dependency_func_map.insert(crate::common::servo::COMPONENT_NAME.into(), Map::new());
        dependency_func_map.insert(
            crate::common::power_sensor::COMPONENT_NAME.into(),
            Map::new(),
        );
        dependency_func_map.insert(crate::common::generic::COMPONENT_NAME.into(), Map::new());
        Self {
            motors: Map::new(),
            board: Map::new(),
            buttons: Map::new(),
            #[cfg(feature = "camera")]
            camera: Map::new(),
            sensor: Map::new(),
            movement_sensors: Map::new(),
            encoders: Map::new(),
            bases: Map::new(),
            servos: Map::new(),
            switches: Map::new(),
            power_sensors: Map::new(),
            generic_components: Map::new(),
            dependencies: dependency_func_map,
        }
    }
    pub fn register_button(
        &mut self,
        model: impl Into<String>,
        constructor: &'static ButtonConstructor,
    ) -> Result<(), RegistryError> {
        let model = model.into();
        if self.buttons.contains_key(&model) {
            return Err(RegistryError::ModelAlreadyRegistered(model));
        }
        let _ = self.buttons.insert(model, constructor);
        Ok(())
    }
    #[cfg(feature = "camera")]
    pub fn register_camera(
        &mut self,
        model: impl Into<String>,
        constructor: &'static CameraConstructor,
    ) -> Result<(), RegistryError> {
        let model = model.into();
        if self.camera.contains_key(&model) {
            return Err(RegistryError::ModelAlreadyRegistered(model));
        }
        let _ = self.camera.insert(model, constructor);
        Ok(())
    }
    pub fn register_motor(
        &mut self,
        model: impl Into<String>,
        constructor: &'static MotorConstructor,
    ) -> Result<(), RegistryError> {
        let model = model.into();
        if self.motors.contains_key(&model) {
            return Err(RegistryError::ModelAlreadyRegistered(model));
        }
        let _ = self.motors.insert(model, constructor);
        Ok(())
    }

    pub fn register_sensor(
        &mut self,
        model: impl Into<String>,
        constructor: &'static SensorConstructor,
    ) -> Result<(), RegistryError> {
        let model = model.into();
        if self.sensor.contains_key(&model) {
            return Err(RegistryError::ModelAlreadyRegistered(model));
        }
        let _ = self.sensor.insert(model, constructor);
        Ok(())
    }

    pub fn register_movement_sensor(
        &mut self,
        model: impl Into<String>,
        constructor: &'static MovementSensorConstructor,
    ) -> Result<(), RegistryError> {
        let model = model.into();
        if self.movement_sensors.contains_key(&model) {
            return Err(RegistryError::ModelAlreadyRegistered(model));
        }
        let _ = self.movement_sensors.insert(model, constructor);
        Ok(())
    }

    pub fn register_board(
        &mut self,
        model: impl Into<String>,
        constructor: &'static BoardConstructor,
    ) -> Result<(), RegistryError> {
        let model = model.into();
        if self.board.contains_key(&model) {
            return Err(RegistryError::ModelAlreadyRegistered(model));
        }
        let _ = self.board.insert(model, constructor);
        Ok(())
    }

    pub fn register_encoder(
        &mut self,
        model: impl Into<String>,
        constructor: &'static EncoderConstructor,
    ) -> Result<(), RegistryError> {
        let model = model.into();
        if self.encoders.contains_key(&model) {
            return Err(RegistryError::ModelAlreadyRegistered(model));
        }
        let _ = self.encoders.insert(model, constructor);
        Ok(())
    }

    pub fn register_base(
        &mut self,
        model: impl Into<String>,
        constructor: &'static BaseConstructor,
    ) -> Result<(), RegistryError> {
        let model = model.into();
        if self.bases.contains_key(&model) {
            return Err(RegistryError::ModelAlreadyRegistered(model));
        }
        let _ = self.bases.insert(model, constructor);
        Ok(())
    }

    pub fn register_power_sensor(
        &mut self,
        model: impl Into<String>,
        constructor: &'static PowerSensorConstructor,
    ) -> Result<(), RegistryError> {
        let model = model.into();
        if self.power_sensors.contains_key(&model) {
            return Err(RegistryError::ModelAlreadyRegistered(model));
        }
        let _ = self.power_sensors.insert(model, constructor);
        Ok(())
    }

    pub fn register_servo(
        &mut self,
        model: impl Into<String>,
        constructor: &'static ServoConstructor,
    ) -> Result<(), RegistryError> {
        let model = model.into();
        if self.servos.contains_key(&model) {
            return Err(RegistryError::ModelAlreadyRegistered(model));
        }
        let _ = self.servos.insert(model, constructor);
        Ok(())
    }

    pub fn register_switch(
        &mut self,
        model: impl Into<String>,
        constructor: &'static SwitchConstructor,
    ) -> Result<(), RegistryError> {
        let model = model.into();
        if self.switches.contains_key(&model) {
            return Err(RegistryError::ModelAlreadyRegistered(model));
        }
        let _ = self.switches.insert(model, constructor);
        Ok(())
    }

    pub fn register_generic_component(
        &mut self,
        model: impl Into<String>,
        constructor: &'static GenericComponentConstructor,
    ) -> Result<(), RegistryError> {
        let model = model.into();
        if self.generic_components.contains_key(&model) {
            return Err(RegistryError::ModelAlreadyRegistered(model));
        }
        let _ = self.generic_components.insert(model, constructor);
        Ok(())
    }

    pub fn register_dependency_getter(
        &mut self,
        component_type: &str,
        model: impl Into<String>,
        getter: &'static DependenciesFromConfig,
    ) -> Result<(), RegistryError> {
        let model = model.into();
        if !self.dependencies.contains_key(component_type) {
            return Err(RegistryError::ComponentTypeNotInDependencies(
                component_type.to_string(),
            ));
        }
        let comp_deps = self.dependencies.get_mut(component_type).unwrap();
        if comp_deps.contains_key(&model) {
            return Err(RegistryError::ModelDependencyFuncRegistered(model));
        }
        let _ = comp_deps.insert(model, getter);
        Ok(())
    }

    pub(crate) fn get_dependency_function(
        &self,
        component_type: &str,
        model_name: &str,
    ) -> Result<&'static DependenciesFromConfig, RegistryError> {
        if !self.dependencies.contains_key(component_type) {
            return Err(RegistryError::ComponentTypeNotInDependencies(
                component_type.into(),
            ));
        }
        let comp_deps = self.dependencies.get(component_type).unwrap();
        if let Some(func) = comp_deps.get(model_name) {
            return Ok(*func);
        }
        Err(RegistryError::ModelNotFoundInDependencies(
            model_name.into(),
            component_type.into(),
        ))
    }

    pub(crate) fn get_board_constructor(
        &self,
        model: &str,
    ) -> Result<&'static BoardConstructor, RegistryError> {
        if let Some(ctor) = self.board.get(model) {
            return Ok(*ctor);
        }
        Err(RegistryError::ModelNotFound(model.into()))
    }

    pub(crate) fn get_button_constructor(
        &self,
        model: &str,
    ) -> Result<&'static ButtonConstructor, RegistryError> {
        if let Some(ctor) = self.buttons.get(model) {
            return Ok(*ctor);
        }
        Err(RegistryError::ModelNotFound(model.into()))
    }

    #[cfg(feature = "camera")]
    pub(crate) fn get_camera_constructor(
        &self,
        model: &str,
    ) -> Result<&'static CameraConstructor, RegistryError> {
        if let Some(ctor) = self.camera.get(model) {
            return Ok(*ctor);
        }
        Err(RegistryError::ModelNotFound(model.into()))
    }

    pub(crate) fn get_motor_constructor(
        &self,
        model: &str,
    ) -> Result<&'static MotorConstructor, RegistryError> {
        if let Some(ctor) = self.motors.get(model) {
            return Ok(*ctor);
        }
        Err(RegistryError::ModelNotFound(model.into()))
    }

    pub(crate) fn get_sensor_constructor(
        &self,
        model: &str,
    ) -> Result<&'static SensorConstructor, RegistryError> {
        if let Some(ctor) = self.sensor.get(model) {
            return Ok(*ctor);
        }
        Err(RegistryError::ModelNotFound(model.into()))
    }

    pub(crate) fn get_movement_sensor_constructor(
        &self,
        model: &str,
    ) -> Result<&'static MovementSensorConstructor, RegistryError> {
        if let Some(ctor) = self.movement_sensors.get(model) {
            return Ok(*ctor);
        }
        Err(RegistryError::ModelNotFound(model.into()))
    }

    pub(crate) fn get_encoder_constructor(
        &self,
        model: &str,
    ) -> Result<&'static EncoderConstructor, RegistryError> {
        if let Some(ctor) = self.encoders.get(model) {
            return Ok(*ctor);
        }
        Err(RegistryError::ModelNotFound(model.to_string()))
    }

    pub(crate) fn get_base_constructor(
        &self,
        model: &str,
    ) -> Result<&'static BaseConstructor, RegistryError> {
        if let Some(ctor) = self.bases.get(model) {
            return Ok(*ctor);
        }
        Err(RegistryError::ModelNotFound(model.to_string()))
    }

    pub(crate) fn get_power_sensor_constructor(
        &self,
        model: &str,
    ) -> Result<&'static PowerSensorConstructor, RegistryError> {
        if let Some(ctor) = self.power_sensors.get(model) {
            return Ok(*ctor);
        }
        Err(RegistryError::ModelNotFound(model.to_string()))
    }

    pub(crate) fn get_servo_constructor(
        &self,
        model: &str,
    ) -> Result<&'static ServoConstructor, RegistryError> {
        if let Some(ctor) = self.servos.get(model) {
            return Ok(*ctor);
        }
        Err(RegistryError::ModelNotFound(model.to_string()))
    }

    pub(crate) fn get_switch_constructor(
        &self,
        model: &str,
    ) -> Result<&'static SwitchConstructor, RegistryError> {
        if let Some(ctor) = self.switches.get(model) {
            return Ok(*ctor);
        }
        Err(RegistryError::ModelNotFound(model.to_string()))
    }

    pub(crate) fn get_generic_component_constructor(
        &self,
        model: &str,
    ) -> Result<&'static GenericComponentConstructor, RegistryError> {
        if let Some(ctor) = self.generic_components.get(model) {
            return Ok(*ctor);
        }
        Err(RegistryError::ModelNotFound(model.to_string()))
    }
}
#[cfg(test)]
mod tests {
    use crate::common::exec::Executor;
    use crate::common::generic::DoCommand;
    use crate::common::motor::MotorError;
    use crate::google;

    use crate::common::sensor::SensorError;
    use crate::common::{
        self,
        config::{ConfigType, DynamicComponentConfig, Model, ResourceName},
        registry::{ComponentRegistry, Dependency, RegistryError},
        robot::LocalRobot,
        sensor::{
            GenericReadingsResult, Readings, Sensor, SensorResult, SensorT, SensorType,
            TypedReadingsResult,
        },
    };
    use std::sync::{Arc, Mutex};

    pub struct TestSensor {}

    impl TestSensor {
        pub fn new() -> Self {
            Self {}
        }
        pub fn from_config(
            _cfg: ConfigType,
            _: Vec<Dependency>,
        ) -> Result<SensorType, SensorError> {
            Ok(Arc::new(Mutex::new(Self {})))
        }
    }
    impl Default for TestSensor {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Sensor for TestSensor {}

    impl Readings for TestSensor {
        fn get_generic_readings(&mut self) -> Result<GenericReadingsResult, SensorError> {
            Ok(self
                .get_readings()?
                .into_iter()
                .map(|v| (v.0, SensorResult::<f64> { value: v.1 }.into()))
                .collect())
        }
    }

    impl SensorT<f64> for TestSensor {
        fn get_readings(&self) -> Result<TypedReadingsResult<f64>, SensorError> {
            let mut x = std::collections::HashMap::new();
            x.insert("test_sensor".to_string(), 42.0);
            Ok(x)
        }
    }

    impl DoCommand for TestSensor {}

    #[test_log::test]
    fn test_driver() {
        use crate::proto::app::v1::{ComponentConfig, RobotConfig};
        let components = vec![
            ComponentConfig {
                name: "board".to_string(),
                api: "rdk:component:board".to_string(),
                model: "rdk:builtin:fake".to_string(),
                attributes: None,
                ..Default::default()
            },
            ComponentConfig {
                name: "test_sensor".to_string(),
                api: "rdk:component:sensor".to_string(),
                model: "rdk:builtin:test_sensor".to_string(),
                attributes: None,
                ..Default::default()
            },
        ];

        let config: RobotConfig = RobotConfig {
            components,
            ..Default::default()
        };

        let mut registry = ComponentRegistry::new();

        // sensor should not be registered yet
        let ctor = registry.get_sensor_constructor("test_sensor");
        assert!(ctor.is_err());
        assert_eq!(
            ctor.err().unwrap(),
            RegistryError::ModelNotFound("test_sensor".to_string())
        );

        // register fake board
        common::board::register_models(&mut registry);
        let ctor = registry.get_board_constructor("fake");
        assert!(ctor.is_ok());

        // register test sensor
        assert!(registry
            .register_sensor("test_sensor".to_string(), &TestSensor::from_config)
            .is_ok());

        // check ctor
        let ctor = registry.get_sensor_constructor("test_sensor");
        assert!(ctor.is_ok());

        let agent_config = common::config::AgentConfig {
            firmware_mode: common::system::FirmwareMode::Normal,
            ..Default::default()
        };

        // make robot
        let robot = LocalRobot::from_cloud_config(
            Executor::new(),
            "".to_string(),
            &config,
            &mut Box::new(registry),
            None,
            &agent_config,
        );
        assert!(robot.is_ok());
        let robot = robot.unwrap();

        // get test value from sensor
        let test_sensor = robot
            .get_sensor_by_name("test_sensor".to_string())
            .expect("could not find test_sensor");
        let r = test_sensor
            .lock()
            .unwrap()
            .get_generic_readings()
            .unwrap()
            .get("test_sensor")
            .expect("could not get reading")
            .clone();
        assert_eq!(
            r,
            google::protobuf::Value {
                kind: Some(google::protobuf::value::Kind::NumberValue(42.0))
            }
        );
    }

    #[test_log::test]
    fn test_registry() {
        let mut registry = ComponentRegistry::new();

        let ctor = registry.get_motor_constructor("fake");
        assert!(ctor.is_err());
        assert_eq!(
            ctor.err().unwrap(),
            RegistryError::ModelNotFound("fake".into())
        );
        common::motor::register_models(&mut registry);

        let ctor = registry.get_motor_constructor("fake");
        assert!(ctor.is_ok());

        let ret = registry.register_motor("fake", &|_, _| {
            Err(MotorError::MotorMethodUnimplemented(""))
        });
        assert!(ret.is_err());
        assert_eq!(
            ret.err().unwrap(),
            RegistryError::ModelAlreadyRegistered("fake".into())
        );

        let ret = registry.register_motor("fake2", &|_, _| {
            Err(MotorError::MotorMethodUnimplemented(""))
        });
        assert!(ret.is_ok());

        let ctor = registry.get_board_constructor("fake");
        assert!(ctor.is_err());
        assert_eq!(
            ctor.err().unwrap(),
            RegistryError::ModelNotFound("fake".into())
        );
        common::board::register_models(&mut registry);

        let ctor = registry.get_board_constructor("fake");
        assert!(ctor.is_ok());

        let ret = registry.register_board("fake", &|_| {
            Err(common::board::BoardError::BoardMethodNotSupported(""))
        });
        assert!(ret.is_err());
        assert_eq!(
            ret.err().unwrap(),
            RegistryError::ModelAlreadyRegistered("fake".into())
        );

        let ret = registry.register_board("fake2", &|_| {
            Err(common::board::BoardError::BoardMethodNotSupported(""))
        });
        assert!(ret.is_ok());

        let ctor = registry.get_motor_constructor("fake2");
        assert!(ctor.is_ok());

        let ret = ctor.unwrap()(
            ConfigType::Dynamic(&DynamicComponentConfig {
                name: ResourceName::new_builtin(
                    "unimplemented".to_owned(),
                    "unimplemented".to_owned(),
                ),
                model: Model::new_builtin("unimplemented".to_owned()),
                data_collector_configs: vec![],
                attributes: None,
            }),
            Vec::new(),
        );

        assert!(ret.is_err());
        assert_eq!(format!("{}", ret.err().unwrap()), "unimplemented: ");

        let ctor = registry.get_board_constructor("fake2");
        assert!(ctor.is_ok());

        let ret = ctor.unwrap()(ConfigType::Dynamic(&DynamicComponentConfig {
            name: ResourceName::new_builtin("unimplemented".to_owned(), "unimplemented".to_owned()),
            model: Model::new_builtin("unimplemented".to_owned()),
            data_collector_configs: vec![],
            attributes: None,
        }));

        assert!(ret.is_err());
        assert_eq!(format!("{}", ret.err().unwrap()), "method:  not supported");
    }
}
