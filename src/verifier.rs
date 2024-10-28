use winterfell::{
    crypto::{hashers::Blake3_256, DefaultRandomCoin, MerkleTree},
    math::fields::f128::BaseElement,
    verify, AcceptableOptions, Proof,
};

use crate::air::{PublicInputs, WorkAir};

type Blake3 = Blake3_256<BaseElement>;

pub fn verify_work(fb_0: BaseElement, fb_1: BaseElement, fb_n: BaseElement, proof: Proof) {
    let min_opts = AcceptableOptions::MinConjecturedSecurity(95);

    let pub_inputs = PublicInputs { fb_0, fb_1, fb_n };
    match verify::<WorkAir, Blake3, DefaultRandomCoin<Blake3>, MerkleTree<Blake3>>(proof, pub_inputs, &min_opts) {
        Ok(_) => println!("yay! all good!"),
        Err(_) => panic!("something went terribly wrong!"),
    }
}