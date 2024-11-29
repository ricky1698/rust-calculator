pub mod advanced;
pub mod arithmetic;

pub trait Operation {
    fn execute(&self, a: f64, b: f64) -> Result<f64, String>;
}
