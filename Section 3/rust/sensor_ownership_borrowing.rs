use std::thread;
use std::time::Duration;

struct SensorReading {
    temperature: f64,
    humidity: f64,
    timestamp: i64,
}

fn analyze_readings(readings: &Vec<SensorReading>) -> (f64, f64, i64) {
    let mut total_temperature = 0.0;
    let mut total_humidity = 0.0;
    let mut latest_timestamp = 0;

    for reading in readings {
        total_temperature += reading.temperature;
        total_humidity += reading.humidity;
        latest_timestamp = reading.timestamp;
    }

    let count = readings.len() as f64;

    (
        total_temperature / count,
        total_humidity / count,
        latest_timestamp,
    )
}

fn main() {
    let reading_count = 1_000_000;

    println!(
        "Allocating memory for {} sensor readings...",
        reading_count
    );

    {
        let mut readings = Vec::with_capacity(reading_count);

        for i in 0..reading_count {
            readings.push(SensorReading {
                temperature: 20.0 + (i % 15) as f64 * 0.25,
                humidity: 40.0 + (i % 20) as f64 * 0.5,
                timestamp: 1_700_000_000 + i as i64,
            });
        }

        let (average_temperature, average_humidity, latest_timestamp) =
            analyze_readings(&readings);

        println!(
            "Average temperature: {:.2} C",
            average_temperature
        );

        println!(
            "Average humidity: {:.2}%",
            average_humidity
        );

        println!(
            "Latest timestamp: {}",
            latest_timestamp
        );

        println!("The sensor buffer is owned by main.");
        println!(
            "The analyze_readings function borrowed the buffer using &readings."
        );

        println!("Memory is currently allocated.");

        thread::sleep(Duration::from_secs(8));

        println!("Leaving the scope...");
    }

    println!(
        "The sensor buffer went out of scope and Rust automatically released its memory."
    );

    thread::sleep(Duration::from_secs(8));
}