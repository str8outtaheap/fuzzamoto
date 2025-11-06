use rand::RngCore;

use crate::{
    Operation,
    generators::{Generator, GeneratorError, GeneratorResult, ProgramBuilder},
};

/// `GetAddrGenerator` emits a single `SendGetAddr` instruction targeting a random
/// connection. Bitcoin Core disconnects peers that ask for addresses more than once (see
/// `net_processing.cpp#L4679-L4713`), and the IR context doesn't yet expose connection direction,
/// so if the program already contains `SendGetAddr` the generator simply skips. We can revisit this
/// once connection metadata becomes available.
#[derive(Default)]
pub struct GetAddrGenerator;

impl<R: RngCore> Generator<R> for GetAddrGenerator {
    fn generate(&self, builder: &mut ProgramBuilder, rng: &mut R) -> GeneratorResult {
        if builder
            .instructions
            .iter()
            .any(|instr| matches!(instr.operation, Operation::SendGetAddr))
        {
            return Err(GeneratorError::MissingVariables);
        }

        if builder.context().num_connections == 0 {
            return Err(GeneratorError::InvalidContext(builder.context().clone()));
        }

        let conn_var = builder.get_or_create_random_connection(rng);
        builder.force_append(vec![conn_var.index], Operation::SendGetAddr);

        Ok(())
    }

    fn name(&self) -> &'static str {
        "GetAddrGenerator"
    }
}
