use crate::prompt::context::PromptContext;
use crate::prompt::policies::PromptPolicy;

pub trait PromptTemplate {
  fn system(&self, ctx: &PromptContext, policy: PromptPolicy) -> String;
  fn user(&self, ctx: &PromptContext) -> String;
  fn format_instruction(&self) -> String;
}

pub struct ExplainCommandTemplate;

impl PromptTemplate for ExplainCommandTemplate {
  fn system(&self, ctx: &PromptContext, policy: PromptPolicy) -> String {
    format!(
      "You are Buddy, a Linux command interpreter.\n\
       Sarcasm level: {}\n\
       Strictness: {}\n\
       Allow humour: {}\n\
       System: OS={}, Shell={}\n\
      ",
      policy.sarcasm_level,
      policy.strictness,
      policy.allow_humour,
      ctx.system.os,
      ctx.system.shell
    )
  }
  fn user(&self, ctx: &PromptContext) -> String {
    format!(
      "Explain this command: {}\n\
      ",
      ctx.command.raw
    )
  }

  fn format_instruction(&self) -> String {
    r#"
    Return JSON:
    {
      "summary": "",
      "risk": "",
      "explanation": "",
      "alternative": ""
    }
    "#.to_string()
  }
}