use rand::{Rng, RngCore, seq::SliceRandom};

use crate::{
    AddrNetwork, AddrRecord, Generator, GeneratorError, GeneratorResult, Operation, ProgramBuilder,
};

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
        let v1_addrs: Vec<_> = self
            .addresses
            .iter()
            .filter_map(|addr| match addr {
                AddrRecord::V1 { .. } => Some(addr.clone()),
                _ => None,
            })
            .collect();

        if v1_addrs.is_empty() {
            return Err(GeneratorError::InvalidContext(builder.context().clone()));
        }

        let conn_var = builder.get_or_create_random_connection(rng);
        let mut_list = builder.force_append_expect_output(vec![], Operation::BeginBuildAddrList);

        let max_entries = v1_addrs.len().min(16);
        let count = rng.gen_range(1..=max_entries);
        let mut indices: Vec<usize> = (0..v1_addrs.len()).collect();
        indices.shuffle(rng);

        for index in indices.into_iter().take(count) {
            let addr = v1_addrs[index].clone();
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

/// Generates address relay sequences using `addrv2`.
#[derive(Clone, Default)]
pub struct AddrRelayV2Generator {
    addresses: Vec<AddrRecord>,
}

impl AddrRelayV2Generator {
    pub fn new(addresses: Vec<AddrRecord>) -> Self {
        Self { addresses }
    }
}

impl<R: RngCore> Generator<R> for AddrRelayV2Generator {
    fn generate(&self, builder: &mut ProgramBuilder, rng: &mut R) -> GeneratorResult {
        let v2_addrs: Vec<_> = self
            .addresses
            .iter()
            .filter_map(|addr| match addr {
                AddrRecord::V2 {
                    network: AddrNetwork::TorV2,
                    ..
                } => None,
                AddrRecord::V2 { .. } => Some(addr.clone()),
                _ => None,
            })
            .collect();

        if v2_addrs.is_empty() {
            return Err(GeneratorError::InvalidContext(builder.context().clone()));
        }

        let conn_var = builder.get_or_create_random_connection(rng);
        let mut_list = builder.force_append_expect_output(vec![], Operation::BeginBuildAddrListV2);

        let max_entries = v2_addrs.len().min(16);
        let count = rng.gen_range(1..=max_entries);
        let mut indices: Vec<usize> = (0..v2_addrs.len()).collect();
        indices.shuffle(rng);

        for index in indices.into_iter().take(count) {
            let addr = v2_addrs[index].clone();
            let addr_var = builder.force_append_expect_output(vec![], Operation::LoadAddr(addr));
            builder.force_append(vec![mut_list.index, addr_var.index], Operation::AddAddrV2);
        }

        let list_var =
            builder.force_append_expect_output(vec![mut_list.index], Operation::EndBuildAddrListV2);
        builder.force_append(vec![conn_var.index, list_var.index], Operation::SendAddrV2);

        Ok(())
    }

    fn name(&self) -> &'static str {
        "AddrRelayV2Generator"
    }
}
