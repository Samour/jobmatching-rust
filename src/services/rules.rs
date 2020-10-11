use crate::collections::CappedHeap;
use crate::dto::{RuleConfigDto, RuleResultDto};
use crate::engine::config::EvaluationContext;
use crate::engine::match_rating::MatchRating;

pub struct MatchScore {
  pub rating: f64,
  pub details: Vec<RuleResultDto>,
}

pub trait RulesService {
  fn get_rule_configs(&self) -> Vec<RuleConfigDto>;
  fn score_job_for_worker(&self, ctx: &EvaluationContext) -> MatchScore;
  fn score_entries<'a, 'b, 'c, 'd>(
    &self,
    ctxs: &'a Vec<EvaluationContext<'b, 'c, 'd>>,
    limit: u32,
  ) -> Vec<(&'a EvaluationContext<'b, 'c, 'd>, MatchScore)>;
  fn count_satisfied(&self, ctxs: &Vec<EvaluationContext>) -> usize;
}

pub struct RulesServiceImpl {
  match_ratings: Vec<Box<dyn MatchRating + Send + Sync>>,
}

impl RulesServiceImpl {
  pub fn new(match_ratings: Vec<Box<dyn MatchRating + Send + Sync>>) -> RulesServiceImpl {
    RulesServiceImpl { match_ratings }
  }
}

impl RulesService for RulesServiceImpl {
  fn get_rule_configs(&self) -> Vec<RuleConfigDto> {
    self
      .match_ratings
      .iter()
      .map(|r| RuleConfigDto {
        name: String::from(r.get_name()),
        weight: r.get_weight(),
      })
      .collect()
  }

  fn score_job_for_worker(&self, ctx: &EvaluationContext) -> MatchScore {
    log::debug!(
      "Calculating score for Worker {} and Job {}",
      ctx.worker.user_id,
      ctx.job.job_id
    );
    let mut score = MatchScore {
      rating: 0.0,
      details: Vec::new(),
    };
    for match_rating in &self.match_ratings {
      let result = match_rating.determine_rating(&ctx);
      score.details.push(RuleResultDto {
        rule_name: String::from(match_rating.get_name()),
        weight: match_rating.get_weight(),
        rating: result.rating,
        metrics: result.metrics,
      });
      if result.rating < 0.0 || score.rating < 0.0 {
        score.rating = -1.0;
        if ctx.config.short_circuit_failures {
          log::debug!("Short-circuit falure; not running further rules");
          break;
        }
      } else {
        score.rating += result.rating;
      }
    }

    log::debug!("Score calculated: {}", score.rating);
    score
  }

  fn score_entries<'a, 'b, 'c, 'd>(
    &self,
    ctxs: &'a Vec<EvaluationContext<'b, 'c, 'd>>,
    limit: u32,
  ) -> Vec<(&'a EvaluationContext<'b, 'c, 'd>, MatchScore)> {
    log::debug!("Scoring & ranking matches");

    let mut result_heap: CappedHeap<(&EvaluationContext, MatchScore)> =
      CappedHeap::new(limit as usize);
    for ctx in ctxs {
      let result = self.score_job_for_worker(&ctx);
      if result.rating >= 0.0 {
        result_heap.push(result.rating, (ctx, result));
      }
    }

    let mut results: Vec<(&EvaluationContext, MatchScore)> = Vec::new();
    loop {
      match result_heap.pop() {
        Some(n) => results.push(n),
        None => break,
      }
    }
    results.reverse();

    results
  }

  fn count_satisfied(&self, ctxs: &Vec<EvaluationContext>) -> usize {
    log::debug!("Counting matching entries");

    ctxs
      .iter()
      .map(|c| self.score_job_for_worker(c))
      .map(|m| m.rating)
      .filter(|r| *r >= 0.0)
      .count()
  }
}
