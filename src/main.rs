use crate::model::qubit_state::QubitState;

mod model;

fn main() {
  let q = QubitState::zero();
  q.display_bra_ket();
}
