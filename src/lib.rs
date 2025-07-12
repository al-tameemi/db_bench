fn send_to_influxdb3(client: &reqwest::blocking::Client, data: &str) {
    client.post("http://localhost:8086/api/v3/write_lp")
        .header("Content-Type", "text/plain; charset=utf-8")
        .body(data.to_string())
        .send()
        .expect("Failed to send data to InfluxDB");
}