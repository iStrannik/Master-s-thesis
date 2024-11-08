
use winterfell::{
    crypto::{hashers::Blake3_256, DefaultRandomCoin, MerkleTree}, math::{fields::f128::BaseElement, FieldElement}, matrix::ColMatrix, AuxRandElements, DefaultConstraintEvaluator, DefaultTraceLde, FieldExtension, PartitionOptions, Proof, ProofOptions, Prover, StarkDomain, Trace, TraceInfo, TracePolyTable, TraceTable
};

use crate::{air::{PublicInputs, WorkAir}, utils, air::COUNT};

type Blake3 = Blake3_256<BaseElement>;

pub struct WorkProver {
    options: ProofOptions,
}

impl WorkProver {
    pub fn new(options: ProofOptions) -> Self {
        Self { options }
    }
}

impl Prover for WorkProver {
    type BaseField = BaseElement;
    type Air = WorkAir;
    type Trace = TraceTable<BaseElement>;
    type HashFn = Blake3;
    type VC = MerkleTree<Self::HashFn>;
    type RandomCoin = DefaultRandomCoin<Blake3>;
    type TraceLde<E: FieldElement<BaseField = BaseElement>> = DefaultTraceLde<E, Self::HashFn, Self::VC>;
    type ConstraintEvaluator<'a, E: FieldElement<BaseField = BaseElement>> =
        DefaultConstraintEvaluator<'a, WorkAir, E>;

    fn get_pub_inputs(&self, trace: &Self::Trace) -> PublicInputs {
        let last_step = trace.length() - 1;
        PublicInputs {
            fb_start: core::array::from_fn(|i| trace.get(i, 0)),
            fb_n: trace.get(0, last_step),
        }
    }

    fn new_trace_lde<E: FieldElement<BaseField = Self::BaseField>>(
        &self,
        trace_info: &TraceInfo,
        main_trace: &ColMatrix<Self::BaseField>,
        domain: &StarkDomain<Self::BaseField>,
        partition_option: PartitionOptions,
    ) -> (Self::TraceLde<E>, TracePolyTable<E>) {
        DefaultTraceLde::new(trace_info, main_trace, domain, partition_option)
    }

    fn new_evaluator<'a, E: FieldElement<BaseField = BaseElement>>(
        &self,
        air: &'a WorkAir,
        aux_rand_elements: Option<AuxRandElements<E>>,
        composition_coefficients: winterfell::ConstraintCompositionCoefficients<E>,
    ) -> Self::ConstraintEvaluator<'a, E> {
        DefaultConstraintEvaluator::new(air, aux_rand_elements, composition_coefficients)
    }

    fn options(&self) -> &ProofOptions {
        &self.options
    }
}


pub fn build_do_work_trace(fb_start: [BaseElement; COUNT], n: usize) -> TraceTable<BaseElement> {
    let trace_width = COUNT;
    let mut trace = TraceTable::new(trace_width, n);

    trace.fill(
        |state| {
            for i in 0..COUNT {
                state[i] = fb_start[i];
            }
        },
        |_, state| {
            let mut sum = state[COUNT - 1];
            for i in 0..COUNT-1 {
                sum += state[i];
                state[i] = state[i + 1];
            }
            state[COUNT - 1] = sum;
        },
    );

    trace
}

pub fn prove_work(fb_start: [BaseElement; COUNT], n: usize, display_table: bool) -> (BaseElement, Proof) {
    let trace = build_do_work_trace(fb_start, n);
    if display_table {
        utils::display_trace(&trace);
    }
    let result = trace.get(0, n - 1);

    let options = ProofOptions::new(
        32,
        8,
        0,
        FieldExtension::None,
        8,
        127,
    );

    let prover = WorkProver::new(options);
    let proof = prover.prove(trace).unwrap();

    (result, proof)
}
