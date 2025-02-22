#![no_main]

use libfuzzer_sys::fuzz_target;
use docker_image::DockerImage;

fuzz_target!(|data: &[u8]| {
    if let Ok(str) = std::str::from_utf8(data) {
        let _ = DockerImage::parse(str);
    }
});
