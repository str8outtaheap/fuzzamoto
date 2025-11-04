use rand::{Rng, RngCore, seq::SliceRandom};

use crate::{AddrRecord, Generator, GeneratorError, GeneratorResult, Operation, ProgramBuilder};

/// Generates address relay sequences (`SendAddr`).
#[derive(Clone, Default)]
pub struct AddrRelayGenerator {
    addresses: Vec<AddrRecord>,
}

impl AddrRelayGenerator {
    pub fn new(addresses: Vec<AddrRecord>) -> Self {
        Self { addresses }
    }
}

impl<R: RngCore> Generator<R> for AddrRelayGenerator {
    fn generate(&self, builder: &mut ProgramBuilder, rng: &mut R) -> GeneratorResult {
        if self.addresses.is_empty() {
            return Err(GeneratorError::InvalidContext(builder.context().clone()));
        }

        let conn_var = builder.get_or_create_random_connection(rng);
        let mut_list = builder.force_append_expect_output(vec![], Operation::BeginBuildAddrList);

        let max_entries = self.addresses.len().min(16);
        let count = rng.gen_range(1..=max_entries);
        let mut indices: Vec<usize> = (0..self.addresses.len()).collect();
        indices.shuffle(rng);

        for index in indices.into_iter().take(count) {
            let addr = self.addresses[index].clone();
            let addr_var = builder.force_append_expect_output(vec![], Operation::LoadAddr(addr));
            builder.force_append(vec![mut_list.index, addr_var.index], Operation::AddAddr);
        }

        let list_var =
            builder.force_append_expect_output(vec![mut_list.index], Operation::EndBuildAddrList);
        builder.force_append(vec![conn_var.index, list_var.index], Operation::SendAddr);

        Ok(())
    }

    fn name(&self) -> &'static str {
        "AddrRelayGenerator"
    }
}
