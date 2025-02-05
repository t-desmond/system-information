mod system_info;
use  system_info::SystemInfo;
fn main() {
    match SystemInfo::work_dir() {
        Ok(dir) => dir,
        Err(_) => println!("error parsing working directory")
    }
    SystemInfo::cpu_cores();
    SystemInfo::os_name();
}