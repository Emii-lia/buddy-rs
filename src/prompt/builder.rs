use crate::prompt::context::PromptContext;
use crate::prompt::policies::PromptPolicy;
use crate::prompt::templates::PromptTemplate;

pub struct Prompt {
  pub system: String,
  pub user: String,
  pub format: String,
}

pub struct PromptBuilder {
  ctx: PromptContext,
  policy: PromptPolicy,
}

impl PromptBuilder {
  pub fn new(ctx: PromptContext) -> PromptBuilder {
    PromptBuilder {
      ctx,
      policy: PromptPolicy::default(),
    }
  }
  pub fn policy(mut self, policy: PromptPolicy) -> PromptBuilder {
    self.policy = policy;
    self
  }
  pub fn build<T: PromptTemplate>(self, template: T) -> Prompt {
    Prompt {
      system: template.system(&self.ctx, self.policy),
      user: template.user(&self.ctx),
      format: template.format_instruction(),
    }
  }
}

impl Prompt {
  pub fn render(&self) -> String {
    format!(
      "<|system|>\n{}\n<|user|>\n{}\n<|assistant|>",
      self.system, self.user
    )
  }
}