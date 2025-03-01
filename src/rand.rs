use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;

/// Creates a new instance of the RNG seeded via getrandom.
/// Consolidates choice of random number generator algorithm to a single place.
///
/// Currently uses the Pcg64 algorithm for its statistical properties in the
/// use cases present in the neighboring modules. Ideal to get better
/// statistical randomness as crypto needs are not required.
///
/// ```
/// use rand::Rng;
///
/// let mut rng = rand_utils::rng_from_os_rng();
/// let x: u32 = rng.random();
/// ```
#[must_use]
pub fn rng_from_os_rng() -> impl Rng {
    Pcg64::from_os_rng()
}
