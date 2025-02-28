use marlin::spade::prelude::*;
use snafu::Whatever;

#[snafu::report]
fn main() -> Result<(), Whatever> {
    let mut runtime = SpadeRuntime::new(SpadeRuntimeOptions {
        call_swim_build: false,
        ..Default::default()
    })?;

    // testing code here...

    Ok(())
}
