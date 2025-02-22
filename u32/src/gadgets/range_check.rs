use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2_field::extension_field::Extendable;

use crate::gadgets::arithmetic_u32::U32Target;
use crate::gates::range_check_u32::U32RangeCheckGate;

pub fn range_check_u32_circuit<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    vals: Vec<U32Target>,
) {
    let num_input_limbs = vals.len();
    let gate = U32RangeCheckGate::<F, D>::new(num_input_limbs);
    let gate_index = builder.add_gate(gate, vec![]);

    for i in 0..num_input_limbs {
        builder.connect(
            Target::wire(gate_index, gate.wire_ith_input_limb(i)),
            vals[i].0,
        );
    }
}
