use std::env::home_dir;
use regex::Regex;

pub mod detect;
#[derive(Debug)]
pub enum Shell {
  Fish,
  Bash,
  Zsh
}

impl Shell {
  pub fn to_string(&self) -> String {
    match self {
      Shell::Fish => "fish",
      Shell::Bash => "bash",
      Shell::Zsh => "zsh"
    }.to_string()
  }

  pub fn load_config(&self) -> &'static str {
    match self {
      Shell::Fish => include_str!("../../../assets/buddy.fish"),
      Shell::Bash => include_str!("../../../assets/buddy.bash"),
      Shell::Zsh => include_str!("../../../assets/buddy.zsh")
    }
  }

  pub fn is_installed(&self) -> bool {
    match self {
      Shell::Fish => {
        let fish_conf = home_dir().unwrap().join(".config/fish/functions/buddy.fish");
        fish_conf.exists()
      }
      Shell::Bash => {
        let bashrc = home_dir().unwrap().join(".bashrc");
        let re = Regex::new(r"# >>> buddy >>>[\s\S]*?# <<< buddy <<<").unwrap();
        let bashrc_content = std::fs::read_to_string(&bashrc).expect("Failed to read bashrc");
        re.is_match(&bashrc_content)
      }
      Shell::Zsh => {
        let zshrc = home_dir().unwrap().join(".zshrc");
        let re = Regex::new(r"# >>> buddy >>>[\s\S]*?# <<< buddy <<<").unwrap();
        let zshrc_content = std::fs::read_to_string(&zshrc).expect("Failed to read zshrc");
        re.is_match(&zshrc_content)
      }
    }
  }

  pub fn install(&self) {
    if self.is_installed() {
      println!("Buddy is already installed");
      return;
    }
    let config = self.load_config();
    match self {
      Shell::Fish => {
        println!("Installing fish config...");
        let fish_conf = home_dir().unwrap().join(".config/fish/functions/buddy.fish");
        if !fish_conf.exists() {
          std::fs::create_dir_all(fish_conf.parent().unwrap()).expect("Failed to create fish conf.d directory");
        }
        std::fs::write(fish_conf, config).expect("Failed to write fish config")
      }
      Shell::Bash => {
        println!("Installing bash config...");
        let bash_config = home_dir().unwrap().join(".config/buddy/buddy.bash");
        std::fs::write(bash_config, config).expect("Failed to write bash config");

        let source_line = "\n# >>> buddy >>>\n[[ -f ~/.config/buddy/buddy.bash ]] && source ~/.config/buddy/buddy.bash\n# <<< buddy <<<\n";

        let bashrc = home_dir().unwrap().join(".bashrc");
        let mut file = std::fs::OpenOptions::new()
          .append(true)
          .create(true)
          .open(bashrc)
          .expect("Failed to open bashrc");
        use std::io::Write;
        file.write_all(source_line.as_bytes()).expect("Failed to write to bashrc");
      }
      Shell::Zsh => {
        println!("Installing zsh config...");
        let zsh_config = home_dir().unwrap().join(".config/buddy/buddy.zsh");
        std::fs::write(zsh_config, config).expect("Failed to write zsh config");

        let source_line = "\n# >>> buddy >>>\n[[ -f ~/.config/buddy/buddy.zsh ]] && source ~/.config/buddy/buddy.zsh\n# <<< buddy <<<\n";

        let zshrc = home_dir().unwrap().join(".zshrc");
        let mut file = std::fs::OpenOptions::new()
          .append(true)
          .create(true)
          .open(zshrc)
          .expect("Failed to open zshrc");
        use std::io::Write;
        file.write_all(source_line.as_bytes()).expect("Failed to write to zshrc");
      }
    }
  }

  pub fn uninstall(&self) {
    match self {
      Shell::Fish => {
        println!("Uninstalling buddy fish config...");
        let fish_function = home_dir().unwrap().join(".config/fish/functions/buddy.fish");
        if fish_function.exists() {
          std::fs::remove_file(fish_function).expect("Failed to remove legacy fish function");
        }
      }
      Shell::Bash => {
        println!("Uninstalling buddy bash config...");
        let bashrc = home_dir().unwrap().join(".bashrc");
        let re = Regex::new(r"# >>> buddy >>>[\s\S]*?# <<< buddy <<<").unwrap();
        let bashrc_content = std::fs::read_to_string(&bashrc).expect("Failed to read bashrc");
        let new_content = re.replace_all(&bashrc_content, "").to_string();
        std::fs::write(bashrc, new_content).expect("Failed to write bashrc");
      }
      Shell::Zsh => {
        println!("Uninstalling buddy zsh config...");
        let zshrc = home_dir().unwrap().join(".zshrc");
        let re = Regex::new(r"# >>> buddy >>>[\s\S]*?# <<< buddy <<<").unwrap();
        let zshrc_content = std::fs::read_to_string(&zshrc).expect("Failed to read zshrc");
        let new_content = re.replace_all(&zshrc_content, "").to_string();
        std::fs::write(zshrc, new_content).expect("Failed to write zshrc");
      }
    }
  }
}