use winterfell::{
    math::{fields::f128::BaseElement, FieldElement, ToElements}, Air, AirContext, Assertion, EvaluationFrame, ProofOptions, TraceInfo, TransitionConstraintDegree
};

pub const COUNT: usize = 254;

pub struct PublicInputs {
    pub(crate) fb_start: [BaseElement; COUNT],
    pub(crate) fb_n: BaseElement,
}

impl ToElements<BaseElement> for PublicInputs {
    fn to_elements(&self) -> Vec<BaseElement> {
        let mut a = Vec::from(self.fb_start);
        a.extend([self.fb_n]);
        a
    }
}

pub struct WorkAir {
    context: AirContext<BaseElement>,
    fb_start: [BaseElement; COUNT],
    fb_n: BaseElement,
}

impl Air for WorkAir {
    type BaseField = BaseElement;
    type PublicInputs = PublicInputs;

    fn new(trace_info: TraceInfo, pub_inputs: PublicInputs, options: ProofOptions) -> Self {
        assert_eq!(COUNT, trace_info.width());

        let degrees = vec![TransitionConstraintDegree::new(1); COUNT];

        let num_assertions = COUNT + 1;

        WorkAir {
            context: AirContext::new(trace_info, degrees, num_assertions, options),
            fb_start: pub_inputs.fb_start,
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
        let mut next_state = current_state[0];
        for i in 1..COUNT {
            next_state += current_state[i];
        }
        let next_state_in_trace = frame.next();

        // println!("{}", result.len());
        // println!("{}", next_state_in_trace.len());
        for i in 0..COUNT-1 {
            result[i] = next_state_in_trace[i] - current_state[i + 1]
        }
        result[COUNT - 1] = next_state_in_trace[COUNT - 1] - next_state;
    }

    fn get_assertions(&self) -> Vec<Assertion<Self::BaseField>> {
        let last_step = self.trace_length() - 1;
        let mut result = vec![Assertion::single(0, last_step, self.fb_n)];
        for i in 0..COUNT {
            result.push(Assertion::single(i, 0, self.fb_start[i]));
        }
        result
    }

    fn context(&self) -> &AirContext<Self::BaseField> {
        &self.context
    }

    // Some hack to compile
    type GkrProof = ();
    type GkrVerifier = ();
}

