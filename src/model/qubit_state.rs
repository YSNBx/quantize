use num_complex::Complex64;

#[derive(Debug, Clone)]
pub struct QubitState {
  alpha: Complex64,
  beta: Complex64,
}

impl QubitState {
  pub fn new(alpha: Complex64, beta: Complex64) -> Self {
    let norm = (alpha.norm_sqr() + beta.norm_sqr()).sqrt();
    Self {
      alpha: alpha / norm,
      beta: beta / norm,
    }
  }

  pub fn zero() -> Self {
    Self::new(Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0))
  }

  pub fn one() -> Self {
    Self::new(Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0))
  }

  pub fn display_bra_ket(&self) {
    println!(
      "|ψ⟩ = ({:.3} + {:.3}i)|0⟩ + ({:.3} + {:.3}i)|1⟩",
      self.alpha.re, self.alpha.im, self.beta.re, self.beta.im
    );
  }
}
