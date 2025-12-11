use std::collections::HashMap;
use std::fs::read_to_string;

fn traverse_devices(devices: &HashMap<&str, Vec<&str>>, device: &str, count: &mut i32) {
    if device == "out" {
        *count += 1;
        return;
    }

    for output_device in devices.get(device).unwrap() {
        traverse_devices(devices, output_device, count);
    }
}

fn main() {
    let input_content = read_to_string("../input.txt").expect("Could not open the file");
    let devices: HashMap<&str, Vec<&str>> = input_content
        .lines()
        .map(|line| {
            let (device_name, outputs) = line.split_once(": ").expect("Could not split the line");
            let device_outputs = outputs.split(" ").collect();
            (device_name, device_outputs)
        })
        .collect();

    let mut count: i32 = 0;
    traverse_devices(&devices, "you", &mut count);

    println!("{}", count);
}
