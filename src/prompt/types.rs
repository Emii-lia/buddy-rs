pub enum RiskLevel {
  Low,
  Medium,
  High,
  Critical,
}

pub enum Verbosity {
  Minimal,
  Normal,
  Detailed,
}

pub enum PromptIntent {
  ExplainCommand,
  AnalyzeRisk,
  SuggestAlternative,
  ManPageTranslation,
}

pub enum OutputFormat {
  Text,
  Json,
  Markdown,
}