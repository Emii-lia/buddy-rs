pub struct PromptPolicy {
  pub sarcasm_level: u8,
  pub strictness: u8,
  pub allow_humour: bool,
}

impl Default for PromptPolicy {
  fn default() -> Self {
    PromptPolicy {
      sarcasm_level: 2,
      strictness: 5,
      allow_humour: true,
    }
  }
}
pub enum PolicyType {
  Boss,
  Mochi,
  Buddy,
  MrBruno,
  Gremlin,
  Spock
}

impl PolicyType {
  pub fn to_policy(&self) -> PromptPolicy {
    match self {
      PolicyType::Boss => PromptPolicy {
        sarcasm_level: 1,
        strictness: 10,
        allow_humour: false,
      },
      PolicyType::Mochi => PromptPolicy::default(),
      PolicyType::Buddy => PromptPolicy {
        sarcasm_level: 8,
        strictness: 6,
        allow_humour: true,
      },
      PolicyType::MrBruno => PromptPolicy {
        sarcasm_level: 9,
        strictness: 10,
        allow_humour: true,
      },
      PolicyType::Gremlin => PromptPolicy {
        sarcasm_level: 10,
        strictness: 1,
        allow_humour: true,
      },
      PolicyType::Spock => PromptPolicy {
        sarcasm_level: 3,
        strictness: 7,
        allow_humour: false,
      }
    }
  }
  pub fn buddy(&self) -> &str {
    //TODO buddy emoji for each policy type
    match self {
      PolicyType::Boss => "(｀Д´)ゞ",
      PolicyType::Mochi => "(｡◕‿◕｡)",
      PolicyType::Buddy => "(⌐■_■)",
      PolicyType::MrBruno => "(ಠ‿ಠ)",
      PolicyType::Gremlin => "(¬‿¬✧)",
      PolicyType::Spock => "(￣＿￣)"
    }
  }
}

impl From<String> for PolicyType {
  fn from(s: String) -> Self {
    match s.to_lowercase().as_str() {
      "boss" => PolicyType::Boss,
      "mochi" => PolicyType::Mochi,
      "buddy" => PolicyType::Buddy,
      "mrbruno" => PolicyType::MrBruno,
      "bruno" => PolicyType::MrBruno,
      "gremlin" => PolicyType::Gremlin,
      "spock" => PolicyType::Spock,
      _ => PolicyType::Buddy,
    }
  }
}