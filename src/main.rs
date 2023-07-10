use gethostname::gethostname;
use influxdb::InfluxDbWriteable;
use influxdb::{Client, Timestamp};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{self, Duration};

use nvml_wrapper::{
    enum_wrappers::device::{Clock, TemperatureSensor},
    Nvml,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let username = env::var("INFLUXDB_USERNAME").expect("set INFLUXDB_USERNAME");
    let password = env::var("INFLUXDB_PASSWORD").expect("set INFLUXDB_PASSWORD");
    let hostname = env::var("INFLUXDB_HOSTNAME").unwrap_or("http://homeassistant:8086".to_string());
    let database = env::var("INFLUXDB_DATABASE").unwrap_or("homeassistant".to_string());
    let client = Client::new(hostname, database).with_auth(username, password);

    let nvml = Nvml::init()?;
    let device = nvml.device_by_index(0)?;

    let hn_os = gethostname();
    let hostname = hn_os.to_str().unwrap_or("localhost");

    let interval_length = env::var("NVMON_INTERVAL")
        .unwrap_or("1".to_string())
        .parse::<u64>()?;
    let mut interval = time::interval(Duration::from_secs(interval_length));

    loop {
        let now = SystemTime::now();
        let etime = now.duration_since(UNIX_EPOCH)?.as_millis();

        let points = vec![
            Timestamp::Milliseconds(etime)
                .into_query("gpu_pwr")
                .add_field("value", device.power_usage()? / 1000)
                .add_field("unit", "W")
                .add_tag("hostname", hostname),
            Timestamp::Milliseconds(etime)
                .into_query("gpu_temp")
                .add_field("value", device.temperature(TemperatureSensor::Gpu)?)
                .add_field("unit", "C")
                .add_tag("hostname", hostname),
            Timestamp::Milliseconds(etime)
                .into_query("gpu_mclk")
                .add_field("value", device.clock_info(Clock::Memory)?)
                .add_field("unit", "MHz")
                .add_tag("hostname", hostname),
            Timestamp::Milliseconds(etime)
                .into_query("gpu_pclk")
                .add_field("value", device.clock_info(Clock::Graphics)?)
                .add_field("unit", "MHz")
                .add_tag("hostname", hostname),
            Timestamp::Milliseconds(etime)
                .into_query("gpu_mem_free")
                .add_field("value", device.memory_info()?.free / (1024 * 1024))
                .add_field("unit", "GB")
                .add_tag("hostname", hostname),
            Timestamp::Milliseconds(etime)
                .into_query("gpu_mem_used")
                .add_field("value", device.memory_info()?.used / (1024 * 1024))
                .add_field("unit", "GB")
                .add_tag("hostname", hostname),
        ];

        client.query(points).await?;

        interval.tick().await;
    }
}
