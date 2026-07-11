pub struct PromptPolicy {
  pub name: String,
  pub description: String,
  pub sarcasm_level: u8,
  pub strictness: u8,
  pub allow_humour: bool,
}

impl Default for PromptPolicy {
  fn default() -> Self {
    PromptPolicy {
      name: "Mochi".to_string(),
      description: PolicyType::from(String::from("Mochi")).description().to_string(),
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
        name: "Boss".to_string(),
        description: self.description().to_string(),
        sarcasm_level: 1,
        strictness: 10,
        allow_humour: false,
      },
      PolicyType::Mochi => PromptPolicy::default(),
      PolicyType::Buddy => PromptPolicy {
        name: "Buddy".to_string(),
        description: self.description().to_string(),
        sarcasm_level: 8,
        strictness: 4,
        allow_humour: true,
      },
      PolicyType::MrBruno => PromptPolicy {
        name: "Mr. Bruno".to_string(),
        description: self.description().to_string(),
        sarcasm_level: 9,
        strictness: 10,
        allow_humour: true,
      },
      PolicyType::Gremlin => PromptPolicy {
        name: "Gremlin".to_string(),
        description: self.description().to_string(),
        sarcasm_level: 10,
        strictness: 1,
        allow_humour: false,
      },
      PolicyType::Spock => PromptPolicy {
        name: "Spock".to_string(),
        description: self.description().to_string(),
        sarcasm_level: 3,
        strictness: 7,
        allow_humour: false,
      }
    }
  }
  pub fn buddy(&self) -> &str {
    match self {
      PolicyType::Boss => "(｀Д´)ゞ",
      PolicyType::Mochi => "(｡◕‿◕｡)",
      PolicyType::Buddy => "(⌐■_■)",
      PolicyType::MrBruno => "(ಠ‿ಠ)",
      PolicyType::Gremlin => "(¬‿¬✧)",
      PolicyType::Spock => "(￣＿￣)"
    }
  }
  
  pub fn description(&self) -> &str {
    match self {
      PolicyType::Boss => "A strict, straightforward boss who gives concise, no-nonsense explanations. No humour allowed. He easily gets angry on incompetent employees.",
      PolicyType::Mochi => "A helpful, gentle close friend who provides clear explanations with a touch of humour. Sarcasm is minimal.",
      PolicyType::Buddy => "A bro, a buddy, a friend who gives casual, laid-back explanations with a good dose of sarcasm and humour.",
      PolicyType::MrBruno => "A math teacher who is known for his ability to explain complex concepts in a clear and funny way ; he is strict but always mocks students who solve problems the hard way when there's an easier approach.",
      PolicyType::Gremlin => "A mischievous gremlin who loves to cause chaos and confusion. Provides explanations that are intentionally misleading and sarcastic, with no regard for accuracy or helpfulness.",
      PolicyType::Spock => "A logical and precise Vulcan who provides clear, concise explanations. He is very professional."
    }
  }
}

impl From<String> for PolicyType {
  fn from(s: String) -> Self {
    match s.to_lowercase().as_str() {
      "boss" => PolicyType::Boss,
      "mochi" => PolicyType::Mochi,
      "buddy" => PolicyType::Buddy,
      "mrbruno" | "bruno" => PolicyType::MrBruno,
      "gremlin" => PolicyType::Gremlin,
      "spock" => PolicyType::Spock,
      _ => PolicyType::Buddy,
    }
  }
}