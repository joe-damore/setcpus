fn main() {
    let cpus = num_cpus::get();
    let args: Vec<String> = std::env::args().collect();

    let count_arg = match args.get(1) {
        Some(val) => val.clone(),
        None => {
            println!("No parameter set; resetting to {} CPUs", cpus);
            format!("{}", cpus)
        }
    };

    let count = match count_arg.parse::<u8>() {
        Ok(val) => val,
        Err(_e) => {
            println!("Error parsing argument; '{}' is not a number", count_arg);
            std::process::exit(1);
        }
    };

    if count == 0 {
        println!("Cannot set to 0 CPUs; at least 1 CPU must be enabled.");
        std::process::exit(2);
    };

    if usize::from(count) > cpus {
        println!("Cannot set to {} CPUs; up to {} CPUs can be enabled.", count, cpus);
    };

    let cpu_dir = "/sys/devices/system/cpu";
    for i in 2..(count + 1) {
        let data = format!("{}", 1);
        let cpu_file = format!("{}/cpu{}/online", cpu_dir, i);
        std::fs::write(cpu_file, data).expect("Failed to write CPU file");
    }

    for i in usize::from(count + 1)..cpus {
        let data = format!("{}", 0);
        let cpu_file = format!("{}/cpu{}/online", cpu_dir, i);
        std::fs::write(cpu_file, data).expect("Failed to write CPU file");
    }
}
