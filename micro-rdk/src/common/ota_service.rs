use crate::common::config::{AttributeError, Kind};
use crate::proto::app::v1::ServiceConfig;

/*
ServiceConfig {
        name: "OTA",
        namespace: "rdk",
        r#type: "generic",
        attributes: Some(
            Struct {
                fields: {
                    "url": Value {
                        kind: Some(
                            StringValue(
                            "https://my.bucket.com/my-ota.bin",
                            ),
                        ),
                    },
                },
            },
        ),
        depends_on: [],
        model: "rdk:builtin:ota_service",
        api: "rdk:service:generic",
        service_configs: [],
        log_configuration: None,
}
 */
#[derive(Debug)]
pub(crate) struct OtaService {
    uri: hyper::Uri,
}

impl OtaService {
    pub(crate) fn new(ota_config: &ServiceConfig) -> Self {
        let kind = ota_config.attributes.as_ref().unwrap().fields.get("url").unwrap().kind.clone().unwrap();
        let url = match kind {
            crate::google::protobuf::value::Kind::StringValue(s) => s,
            _ => "".to_string()
        };
        let uri = url.parse::<hyper::Uri>().unwrap();

        Self { uri }
    }

    pub(crate) fn update(&mut self) -> Result<(), String> {

        //    check hash/version/metadata compare to active ota hash/version/metadata
        // get handle to inactive slot
        // start ota process on inactive slot, get write buffer
        // loop through request to url with ota buffer as target
        //   check for FirmwareInfo mid-loop
        //   break when num_read > file_size
        // check firmware info and hash
        // stop writing, release write handle, flip ota bit
        // restart
        Err("unimplemented".to_string())
    }
}
