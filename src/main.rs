use gethostname::gethostname;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{thread, time};

use nvml_wrapper::{
    enum_wrappers::device::{Clock, TemperatureSensor},
    Nvml,
};

#[derive(Serialize, Deserialize)]
struct Point {
    measurement: String,
    tags: HashMap<String, String>,
    fields: HashMap<String, String>,
}

fn main() {
    let nvml = Nvml::init().unwrap();
    let device = nvml.device_by_index(0).unwrap();

    let mut tags = HashMap::new();
    tags.insert("hostname".to_string(), gethostname().into_string().unwrap());

    let fields = HashMap::new();

    let mut point = Point {
        measurement: "".to_string(),
        tags,
        fields,
    };

    loop {
        point.measurement = "pwr".to_string();
        point.fields.insert(
            "value".to_string(),
            (device.power_usage().unwrap() / 1000).to_string(),
        );

        println!("{}", serde_json::to_string(&point).unwrap());

        point.measurement = "gtemp".to_string();
        point.fields.insert(
            "value".to_string(),
            device
                .temperature(TemperatureSensor::Gpu)
                .unwrap()
                .to_string(),
        );

        println!("{}", serde_json::to_string(&point).unwrap());

        point.measurement = "mclk".to_string();
        point.fields.insert(
            "value".to_string(),
            device.clock_info(Clock::Memory).unwrap().to_string(),
        );

        println!("{}", serde_json::to_string(&point).unwrap());

        point.measurement = "pclk".to_string();
        point.fields.insert(
            "value".to_string(),
            device.clock_info(Clock::Graphics).unwrap().to_string(),
        );

        println!("{}", serde_json::to_string(&point).unwrap());

        point.measurement = "free".to_string();
        point.fields.insert(
            "value".to_string(),
            (device.memory_info().unwrap().used / (1024 * 1024)).to_string(),
        );

        println!("{}", serde_json::to_string(&point).unwrap());

        thread::sleep(time::Duration::from_secs(1));
    }
}
