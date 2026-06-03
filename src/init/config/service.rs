use std::env::home_dir;

pub fn create_service() -> anyhow::Result<(), anyhow::Error> {
  println!("Creating service...");
  let buddy_service = home_dir().unwrap().join(".config/systemd/user/buddy.service");
  if buddy_service.exists() {
    println!("Buddy service already exists");
    return Ok(());
  }
  std::fs::create_dir_all(buddy_service.parent().unwrap()).expect("Failed to create buddy service directory");
  let buddy_service_content = include_str!("../../../assets/buddy.service");
  std::fs::write(buddy_service, buddy_service_content).expect("Failed to write buddy service");
  println!("Buddy service created");
  println!("Enabling buddy service...");
  std::process::Command::new("systemctl")
    .arg("--user")
    .arg("daemon-reload")
    .output()
    .expect("Failed to enable buddy service");

  std::process::Command::new("systemctl")
    .arg("--user")
    .arg("enable")
    .arg("buddy.service")
    .output()
    .expect("Failed to enable buddy service");
  std::process::Command::new("systemctl")
    .arg("--user")
    .arg("start")
    .arg("buddy.service")
    .output()
    .expect("Failed to start buddy service");

  println!("Buddy service enabled");
  Ok(())
}

pub fn remove_service() -> anyhow::Result<(), anyhow::Error> {
  println!("Removing service...");

  std::process::Command::new("systemctl")
    .arg("--user")
    .arg("daemon-reload")
    .output()
    .expect("Failed to remove buddy service");

  std::process::Command::new("systemctl")
    .arg("--user")
    .arg("disable")
    .arg("buddy.service")
    .output()
    .expect("Failed to disable buddy service");
  std::process::Command::new("systemctl")
    .arg("--user")
    .arg("stop")
    .arg("buddy.service")
    .output()
    .expect("Failed to stop buddy service");

  let buddy_service = home_dir().unwrap().join(".config/systemd/user/buddy.service");
  if buddy_service.exists() {
    std::fs::remove_file(buddy_service).expect("Failed to remove buddy service");
  }
  println!("Buddy service removed");
  Ok(())
}