use crate::Unified;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct RadiusProfile<'rp> {
  #[derivative(Debug = "ignore")]
  #[allow(dead_code)]
  pub(crate) unified: &'rp Unified,
  pub(crate) site: String,

  pub id: String,
  pub name: String,
}
