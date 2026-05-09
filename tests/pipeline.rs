use hamon::errors::{Context, PipelineError, Result};
use hamon::ext::DecoratorExt;
use hamon::prelude::*;

struct Add(i32);
struct Multiply(i32);
struct ToStringStep;
struct ParseI32;
struct FailStep;

impl Decorator<i32, i32> for Add {
    fn produce(&mut self, input: i32) -> Result<i32> {
        Ok(input + self.0)
    }
}

impl Decorator<i32, i32> for Multiply {
    fn produce(&mut self, input: i32) -> Result<i32> {
        Ok(input * self.0)
    }
}

impl Decorator<i32, String> for ToStringStep {
    fn produce(&mut self, input: i32) -> Result<String> {
        Ok(input.to_string())
    }
}

impl Decorator<String, i32> for ParseI32 {
    fn produce(&mut self, input: String) -> Result<i32> {
        input
            .parse::<i32>()
            .step_err(format!("cannot parse {input:?} as i32"))
    }
}

impl Decorator<i32, i32> for FailStep {
    fn produce(&mut self, _input: i32) -> Result<i32> {
        Err(PipelineError::InvalidTransformation("intentional failure".into()))
    }
}

#[test]
fn basic_pipeline_collects_final_value() {
    let result = Builder::new(10)
        .step(Add(2))
        .step(Multiply(10))
        .collect();

    assert!(matches!(result, Ok(120)));
}

#[test]
fn pipeline_can_change_types_between_steps() {
    let result = Builder::new(10)
        .step(Add(2))
        .step(ToStringStep)
        .step(ParseI32)
        .step(Multiply(3))
        .collect();

    assert!(matches!(result, Ok(36)));
}

#[test]
fn pipeline_stops_and_returns_error_when_a_step_fails() {
    let result = Builder::new(10)
        .step(Add(2))
        .step(FailStep)
        .step(Multiply(10))
        .collect();

    assert!(matches!(
        result,
        Err(PipelineError::InvalidTransformation(message)) if message == "intentional failure"
    ));
}

#[test]
fn context_converts_standard_result_to_pipeline_error() {
    let result = Builder::new("not-a-number".to_string())
        .step(ParseI32)
        .collect();

    assert!(matches!(
        result,
        Err(PipelineError::InvalidTransformation(message))
            if message == "cannot parse \"not-a-number\" as i32"
    ));
}

#[test]
fn closures_can_be_used_as_decorators() {
    let result = Builder::new(10)
        .step(|input| -> Result<i32> { Ok(input + 5) })
        .step(|input| -> Result<i32> { Ok(input * 2) })
        .collect();

    assert!(matches!(result, Ok(30)));
}

#[test]
fn conditional_decorator_runs_when_predicate_is_true() {
    let result = Builder::new(10)
        .step(Add(5).when(|input| *input == 10))
        .collect();

    assert!(matches!(result, Ok(15)));
}

#[test]
fn conditional_decorator_is_skipped_when_predicate_is_false() {
    let result = Builder::new(10)
        .step(Add(5).when(|input| *input != 10))
        .collect();

    assert!(matches!(result, Ok(10)));
}

#[test]
fn step_index_tracks_pipeline_depth() {
    let pipeline = Builder::new(10)
        .step(Add(1))
        .step(Add(1))
        .step(Add(1));

    assert_eq!(pipeline.get_index(), 3);
    assert!(matches!(pipeline.collect(), Ok(13)));
}
