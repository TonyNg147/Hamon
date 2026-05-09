use hamon::builder::OrderedBuilder;
use hamon::errors::Result;
use hamon::prelude::*;
use hamon::AllowStep;

struct Encryption;

#[derive(AllowStep)]
#[from(Encryption)]
struct Compression;

impl Decorator<i32, i32> for Encryption {
    fn produce(&mut self, input: i32) -> Result<i32> {
        Ok(input + 1)
    }
}

impl Decorator<i32, i32> for Compression {
    fn produce(&mut self, input: i32) -> Result<i32> {
        Ok(input * 2)
    }
}

#[test]
fn ordered_builder_allows_declared_step_transition() {
    let result = OrderedBuilder::new(10)
        .step(Encryption)
        .step(Compression)
        .collect();

    assert!(matches!(result, Ok(22)));
}

#[test]
fn ordered_step_index_tracks_pipeline_depth() {
    let pipeline = OrderedBuilder::new(10)
        .step(Encryption)
        .step(Compression);

    assert_eq!(pipeline.get_index(), 2);
    assert!(matches!(pipeline.collect(), Ok(22)));
}
