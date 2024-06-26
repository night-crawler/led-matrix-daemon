use std::time::Duration;

pub mod led_matrix_config;
pub mod lef_matrix_config_dto;
pub mod port_dto;

fn yes() -> bool {
    true
}

fn default_baud_rate() -> u32 {
    115200
}

fn default_port_timeout() -> Duration {
    Duration::from_secs(2)
}

fn default_http_workers() -> usize {
    1
}

fn default_max_queue_size() -> usize {
    1
}
