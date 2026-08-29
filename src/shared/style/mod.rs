use colored::{Color, Colorize};

pub fn style_icon(icon: &str, color: Color) -> String {
  icon.color(color).bold().to_string()
}

pub fn style_message(msg: &str) -> String {
  msg.italic().bright_white().to_string()
}

pub fn wrap_in_bubble(msg: &str, buddy: &str) -> String {
  let lines: Vec<&str> = msg.split('\n').collect();
  let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
  let width = max_len + 1;

  let mut bubble = String::new();
  bubble.push_str(&format!("  {}\n", "─".repeat(width + 3).bright_white()));
  for line in lines {
    let padding = " ".repeat(width - line.len() - 1);
    bubble.push_str(&format!("  {} {} {} {}\n",
                             "│".bright_white(),
                             line.italic().bright_white(),
                             padding.bright_white(),
                             "│".bright_white()
    ));
  }
  bubble.push_str(&format!("  {}\n", "─".repeat(width + 3).bright_white()));
  bubble.push_str(&format!(" {} \n", buddy.bold()));
  bubble
}

pub fn wrap_in_box(msg: &str, buddy: &str) -> String {
  let max_width = 60;
  let content_width = max_width - 2;

  let mut wrapped_lines = Vec::new();
  for line in msg.split('\n') {
    if line.is_empty() {
      wrapped_lines.push("".to_string());
      continue;
    }

    let mut current_line = String::new();
    for word in line.split_whitespace() {
      if current_line.is_empty() {
        current_line.push_str(word);
      } else if current_line.len() + word.len() <= content_width {
        current_line.push(' ');
        current_line.push_str(word);
      } else {
        wrapped_lines.push(current_line);
        current_line = word.to_string();
      }
    }
    wrapped_lines.push(current_line);
  }

  let max_len = wrapped_lines.iter().map(|l| l.len()).max().unwrap_or(0);
  let width = max_len + 2;

  let mut boxed = String::new();
  boxed.push_str(&format!("  ╔{}╗\n", "═".repeat(width).bright_white()));
  for line in wrapped_lines {
    let padding = " ".repeat(width - line.len() - 1);
    boxed.push_str(&format!("    {}{}\n",
                             line.bright_white(),
                             padding
    ));
  }
  boxed.push_str(&format!("  ╚{}╝\n", "═".repeat(width).bright_white()));
  boxed.push_str(&format!("   {}\n", buddy.bold()));
  boxed
}

#[cfg(test)]
mod test;
