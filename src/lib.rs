const INFLUXDB_API_KEY: Result<&str, std::str::Utf8Error> = str::from_utf8(include_bytes!("../influxdb_api_key.txt"));

#[allow(unused)]
fn send_to_influxdb3(client: &reqwest::blocking::Client, data: String) {
    client.post("http://localhost:8181/api/v3/write_lp")
        .query(&[("db", "test_db"), ("precision", "auto"), ("no_sync", "true")])
        .bearer_auth(INFLUXDB_API_KEY.expect("Failed to read InfluxDB API key"))
        .body(data)
        .send()
        .expect("Failed to send data to InfluxDB");
}


#[cfg(test)]
mod tests {
    use super::*;
    const INFLUXDB_OPTIMUM_BATCH_SIZE: usize = 10_000; // Optimal batch size for sending data
    const INFLUXDB_OPTIMUM_MESSAGE_SIZE: usize = 10 * 2_usize.pow(20); // 10 MB
    const QUESTDB_OPTIMUM_MESSAGE_SIZE: usize = 2 * 2_usize.pow(20); // 2 MB
    
    #[test]
    fn test_send_to_influxdb3() {
        let client = reqwest::blocking::ClientBuilder::new().gzip(true).build().unwrap();
        let count: usize = 10_000_000; // 1 million records
        let mut data = String::new();
        let start_time = std::time::Instant::now();
        for i in 0..count {
            data.push_str("measurement,tag=value,tag2=value2 field=1,field2=2 1234567890\n");

            if i % INFLUXDB_OPTIMUM_BATCH_SIZE == 0 || data.len() >= INFLUXDB_OPTIMUM_MESSAGE_SIZE {
                // Send data in chunks of 10,000 lines or when the data size exceeds 10 MB
                send_to_influxdb3(&client, data.clone());
                data.clear();
            }
        }

        if data.len() > 0 {
            // Send any remaining data
            send_to_influxdb3(&client, data);
        }

        let elapsed = start_time.elapsed();
        println!("Sent {} records in {:?}", count, elapsed);
        println!("Average time per record: {:?}", elapsed / count as u32);
        println!("Speed: {:.2} records/sec", count as f64 / elapsed.as_secs_f64());
    }

    #[test]
    fn test_send_to_questdb() {
        let mut client = questdb::ingress::Sender::from_conf("http::addr=127.0.0.1:9000;").unwrap();
        let count: usize = 10_000_000; // 1 million records
        let mut data = questdb::ingress::Buffer::new(questdb::ingress::ProtocolVersion::V2);
        let start_time = std::time::Instant::now();
        for _ in 0..count {
            data.table("test_table").unwrap().symbol("tag", "value").unwrap()
                .symbol("tag2", "value2").unwrap()
                .column_i64("field", 1).unwrap()
                .column_i64("field2", 2).unwrap()
                .at_now().unwrap();
            if data.len() >= QUESTDB_OPTIMUM_MESSAGE_SIZE {
                // Send data in chunks of 10,000 lines or when the data size exceeds 2 MB
                client.flush(&mut data).unwrap();
            }
        }

        if data.len() > 0 {
            // Send any remaining data
            client.flush(&mut data).unwrap();
        }

        let elapsed = start_time.elapsed();
        println!("Sent {} records in {:?}", count, elapsed);
        println!("Average time per record: {:?}", elapsed / count as u32);
        println!("Speed: {:.2} records/sec", count as f64 / elapsed.as_secs_f64());
    }
}