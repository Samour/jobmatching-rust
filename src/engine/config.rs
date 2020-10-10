use crate::dto::{JobDto, WorkerDto};

pub struct EvaluationConfig {
  pub with_diagnosis: bool,
  pub short_circuit_failures: bool,
}

pub struct EvaluationContext<'w, 'j, 'c> {
  pub worker: &'w WorkerDto,
  pub job: &'j JobDto,
  pub config: &'c EvaluationConfig,
}

impl EvaluationContext<'_, '_, '_> {
  pub fn new<'w, 'j, 'c>(
    worker: &'w WorkerDto,
    job: &'j JobDto,
    config: &'c EvaluationConfig,
  ) -> EvaluationContext<'w, 'j, 'c> {
    EvaluationContext {
      worker,
      job,
      config,
    }
  }
}
