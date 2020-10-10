use crate::dto::{RuleConfigDto, RuleResultDto};
use crate::engine::config::EvaluationContext;
use crate::engine::match_rating::MatchRating;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct MatchScore {
  pub rating: f64,
  pub details: Vec<RuleResultDto>,
}

fn match_score_cmp<T>(a: &(T, MatchScore), b: &(T, MatchScore)) -> Ordering {
  if a.1.rating > b.1.rating {
    Ordering::Less
  } else if a.1.rating < b.1.rating {
    Ordering::Greater
  } else {
    Ordering::Equal
  }
}

pub trait RulesService {
  fn get_rule_configs(&self) -> Vec<RuleConfigDto>;
  fn score_job_for_worker(&self, ctx: &EvaluationContext) -> MatchScore;
  fn score_entries<'a, 'b, 'c, 'd>(
    &self,
    ctxs: &'a Vec<EvaluationContext<'b, 'c, 'd>>,
    limit: u32,
  ) -> Vec<(&'a EvaluationContext<'b, 'c, 'd>, MatchScore)>;
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
    for match_rating in self.match_ratings.iter() {
      let result = match_rating.determine_rating(&ctx);
      if result.rating < 0.0 || score.rating < 0.0 {
        score.rating = -1.0;
      } else {
        score.rating += result.rating;
      }
      score.details.push(RuleResultDto {
        rule_name: String::from(match_rating.get_name()),
        weight: match_rating.get_weight(),
        rating: result.rating,
        metrics: result.metrics,
      })
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
    let mut matches: Vec<(&EvaluationContext, MatchScore)> = ctxs
      .iter()
      .map(|c| (c, self.score_job_for_worker(&c)))
      .collect();
    // TODO instead of scoring & collecting every job before sorting, instead push each element onto a max heap
    // as they are scored. Set the max heap size = job_limit; after each push, if the heap exceeds that size, drop
    // the smallest value. We can also store the current lowest score in the heap to short-circuit pushing then dropping
    // too-small values.
    // This will drastically cut down on the number of comparisons that need to be made compared to sorting the full
    // list. Additionally, we can reduce the memory footprint somewhat (althoug this is less relevent as find_all_jobs
    // already loads all data into memory anyway)
    matches.sort_by(match_score_cmp);

    log::debug!("Job scoring complete");
    matches[..(limit as usize)]
      .into_iter()
      .map(|j| (j.0, j.1.clone()))
      .collect()
  }
}
