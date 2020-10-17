use crate::ahp::Error as AHPError;
use crate::mpc::MPCError;

/// A `enum` specifying the possible failure modes of the `SNARK`.
#[derive(Debug)]
pub enum Error<E> {
    /// The index is too large for the universal public parameters.
    IndexTooLarge,
    /// There was an error in the underlying holographic IOP.
    HolographicIOPError(AHPError),
    /// There was an error in the underlying polynomial commitment.
    PolynomialCommitmentError(E),
    /// MPC Errors
    MPCErr(MPCError),
}

impl<E> From<AHPError> for Error<E> {
    fn from(err: AHPError) -> Self {
        Error::HolographicIOPError(err)
    }
}

impl<E> From<MPCError> for Error<E> {
    fn from(err: MPCError) -> Self {
        Error::MPCErr(err)
    }
}

impl<E> Error<E> {
    /// Convert an error in the underlying polynomial commitment scheme
    /// to a `Error`.
    pub fn from_pc_err(err: E) -> Self {
        Error::PolynomialCommitmentError(err)
    }
}
