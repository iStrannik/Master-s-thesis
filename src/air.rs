use winterfell::{
    math::{fields::f128::BaseElement, FieldElement, ToElements}, Air, AirContext, Assertion, EvaluationFrame, ProofOptions, TraceInfo, TransitionConstraintDegree
};

pub struct PublicInputs {
    pub(crate) fb_0: BaseElement,
    pub(crate) fb_1: BaseElement,
    pub(crate) fb_n: BaseElement,
}

impl ToElements<BaseElement> for PublicInputs {
    fn to_elements(&self) -> Vec<BaseElement> {
        vec![self.fb_0, self.fb_1, self.fb_n]
    }
}

pub struct WorkAir {
    context: AirContext<BaseElement>,
    fb_0: BaseElement,
    fb_1: BaseElement,
    fb_n: BaseElement,
}

impl Air for WorkAir {
    type BaseField = BaseElement;
    type PublicInputs = PublicInputs;

    fn new(trace_info: TraceInfo, pub_inputs: PublicInputs, options: ProofOptions) -> Self {
        assert_eq!(2, trace_info.width());

        let degrees = vec![TransitionConstraintDegree::new(1), TransitionConstraintDegree::new(1)];

        let num_assertions = 3;

        WorkAir {
            context: AirContext::new(trace_info, degrees, num_assertions, options),
            fb_0: pub_inputs.fb_0,
            fb_1: pub_inputs.fb_1,
            fb_n: pub_inputs.fb_n,
        }
    }

    fn evaluate_transition<E: FieldElement + From<Self::BaseField>>(
        &self,
        frame: &EvaluationFrame<E>,
        _periodic_values: &[E],
        result: &mut [E],
    ) {
        let current_state = frame.current();
        let next_state = current_state[0] + current_state[1];
        let next_state_in_trace = frame.next();

        // println!("{}", result.len());
        // println!("{}", next_state_in_trace.len());

        result[0] = next_state_in_trace[0] - current_state[1];
        result[1] = next_state_in_trace[1] - next_state;
    }

    fn get_assertions(&self) -> Vec<Assertion<Self::BaseField>> {
        let last_step = self.trace_length() - 1;
        vec![
            Assertion::single(0, 0, self.fb_0),
            Assertion::single(1, 0, self.fb_1),
            Assertion::single(0, last_step, self.fb_n),
        ]
    }

    fn context(&self) -> &AirContext<Self::BaseField> {
        &self.context
    }

    // Some hack to compile
    type GkrProof = ();
    type GkrVerifier = ();
}

