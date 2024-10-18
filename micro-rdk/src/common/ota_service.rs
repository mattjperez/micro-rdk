use crate::proto::app::v1::ServiceConfig;

pub struct OtaService {
    ota_config: ServiceConfig,
}

impl OtaService {
    pub fn new(ota_config: ServiceConfig) -> Self {
        Self { ota_config }
    }

    pub fn update(&mut self) -> Result<(), String> {
        // check stored config for url, validate (uri crate?)
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
