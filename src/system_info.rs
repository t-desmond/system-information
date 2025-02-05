use std::env;

pub struct SystemInfo;

impl SystemInfo{
  pub fn work_dir()  -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("The current directory is: {}", path.display());
    Ok(())
  }

  pub fn os_name(){
    println!("The operating system is: {}", env::consts::OS);
  }

  pub fn cpu_cores(){
    println!("This deveice has {} cpu core(s)", num_cpus::get());
  }
}