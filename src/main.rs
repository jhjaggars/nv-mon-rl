use gethostname::gethostname;
use influxdb::InfluxDbWriteable;
use influxdb::{Client, Timestamp};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{self, Duration};

use nvml_wrapper::{
    enum_wrappers::device::{Clock, TemperatureSensor},
    Nvml,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client =
        Client::new("http://homeassistant:8086", "homeassistant").with_auth("user", "pass");

    let nvml = Nvml::init()?;
    let device = nvml.device_by_index(0)?;

    let hostname = gethostname();
    let mut interval = time::interval(Duration::from_secs(1));

    loop {
        let now = SystemTime::now();
        let etime = now.duration_since(UNIX_EPOCH)?.as_millis();

        let points = vec![
            Timestamp::Milliseconds(etime)
                .into_query("pwr")
                .add_field("value", device.power_usage()? / 1000)
                .add_field("unit", "W")
                .add_tag("hostname", hostname.to_str()),
            Timestamp::Milliseconds(etime)
                .into_query("gtemp")
                .add_field("value", device.temperature(TemperatureSensor::Gpu)?)
                .add_field("unit", "C")
                .add_tag("hostname", hostname.to_str()),
            Timestamp::Milliseconds(etime)
                .into_query("mclk")
                .add_field("value", device.clock_info(Clock::Memory)?)
                .add_field("unit", "MHz")
                .add_tag("hostname", hostname.to_str()),
            Timestamp::Milliseconds(etime)
                .into_query("pclk")
                .add_field("value", device.clock_info(Clock::Graphics)?)
                .add_field("unit", "MHz")
                .add_tag("hostname", hostname.to_str()),
            Timestamp::Milliseconds(etime)
                .into_query("free")
                .add_field("value", device.memory_info()?.free / (1024 * 1024))
                .add_field("unit", "MHz")
                .add_tag("hostname", hostname.to_str()),
        ];

        client.query(points).await?;

        interval.tick().await;
    }
}
