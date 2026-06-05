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
  CLI,
  Assistant,
  Buddy,
  Neutral,
}

impl PolicyType {
  pub fn to_policy(&self) -> PromptPolicy {
    match self {
      PolicyType::CLI => PromptPolicy {
        sarcasm_level: 1,
        strictness: 10,
        allow_humour: false,
      },
      PolicyType::Assistant => PromptPolicy {
        sarcasm_level: 1,
        strictness: 5,
        allow_humour: true,
      },
      PolicyType::Buddy => PromptPolicy {
        sarcasm_level: 8,
        strictness: 6,
        allow_humour: true,
      },
      PolicyType::Neutral => PromptPolicy::default(),
    }
  }
}