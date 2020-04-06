use crate::matrix::Matrix;
use crate::rug::{Integer, Rational};
use crate::vector::{BigVector, Dot, RationalVector, Vector};

use std::cmp::max;

/// Lattice reduction (L² algorithm)
///
/// This implementation uses `BigVector` for the underlying arithmetic operations.
///
/// Arguments:
///  * basis: A generating matrix for the lattice
///  * eta: eta factor of the basis reduction
///  * delta: delta factor of the basis reduction
///
/// The basis is reduced in-place.
///
/// # Panics
/// if delta <= 1/4 or delta >= 1  
/// if eta <= 1/2 or eta > sqrt(delta)
pub fn lattice_reduce(basis: &mut Matrix<BigVector>, eta: f64, delta: f64) {
    assert!(0.25 < delta && delta < 1.);
    assert!(0.5 < eta && eta * eta < delta);
    // Variables
    let (d, _) = basis.dimensions();
    let mut gram: Matrix<BigVector> = Matrix::init(d, d); // Gram matrix (upper triangular)
    let mut r: Matrix<RationalVector> = Matrix::init(d, d); // r_ij matrix
    let mut mu: Matrix<RationalVector> = Matrix::init(d, d); // Gram coefficient matrix

    // Computing Gram matrix
    for i in 0..d {
        for j in 0..=i {
            gram[i][j] = basis[i].dot(&basis[j]);
        }
    }

    let eta_minus = Rational::from_f64((eta + 0.5) / 2.).unwrap();
    let delta_plus = Rational::from_f64((delta + 1.) / 2.).unwrap();

    r[0][0] = Rational::from(&gram[0][0]);

    let mut k = 1;

    while k < d {
        size_reduce(
            k,
            d,
            basis,
            &mut gram,
            &mut mu,
            &mut r,
            Rational::from(&eta_minus),
        );

        let delta_criterion = Rational::from(&delta_plus * &r[k - 1][k - 1]);
        let scalar_criterion = &r[k][k] + Rational::from(&mu[k][k - 1]).square() * &r[k - 1][k - 1];

        // Lovazs condition
        if delta_criterion < scalar_criterion {
            k += 1;
        } else {
            basis.swap(k, k - 1);

            // Updating Gram matrix
            for j in 0..d {
                if j < k {
                    gram[k][j] = basis[k].dot(&basis[j]);
                    gram[k - 1][j] = basis[k - 1].dot(&basis[j]);
                } else {
                    gram[j][k] = basis[k].dot(&basis[j]);
                    gram[j][k - 1] = basis[k - 1].dot(&basis[j]);
                }
            }

            // Updating mu and r
            for i in 0..=k {
                for j in 0..=i {
                    r[i][j] = Rational::from(&gram[i][j])
                        - (0..j)
                            .map(|index| Rational::from(&mu[j][index] * &r[i][index]))
                            .sum::<Rational>();
                    mu[i][j] = Rational::from(&r[i][j] / &r[j][j]);
                }
            }

            k = max(1, k - 1);
        }
    }
}

/// Performs the `eta`-size-reduction of `basis[k]`
///
/// Arguments:
/// * `k`: Index of the column to be `eta`-size-reduced
/// * `d`: The basis dimension
/// * `basis`: A generating matrix for the lattice
/// * `gram`: Gram matrix of `basis`  
/// * `mu`: Gram coefficient matrix
/// * `r`: the r_ij matrix
/// * `eta`: eta factor of the basis reduction
///
/// Note: both `basis` and `gram` are updated by this operation.
fn size_reduce(
    k: usize,
    d: usize,
    basis: &mut Matrix<BigVector>,
    gram: &mut Matrix<BigVector>,
    mu: &mut Matrix<RationalVector>,
    r: &mut Matrix<RationalVector>,
    eta: Rational,
) {
    // Update mu and r
    for i in 0..=k {
        r[k][i] = Rational::from(&gram[k][i])
            - (0..i)
                .map(|index| Rational::from(&mu[i][index] * &r[k][index]))
                .sum::<Rational>();
        mu[k][i] = Rational::from(&r[k][i] / &r[i][i]);
    }

    if (0..k).any(|index| mu[k][index] > eta) {
        for i in (0..k).rev() {
            let (_, x) = Rational::from(&mu[k][i]).fract_round(Integer::new());
            basis[k] = basis[k].sub(&basis[i].mulf(&x));

            // Updating Gram matrix
            for j in 0..d {
                if j < k {
                    gram[k][j] = basis[k].dot(&basis[j]);
                } else {
                    gram[j][k] = basis[k].dot(&basis[j]);
                }
            }

            for j in 0..i {
                let shift = Rational::from(&mu[i][j]);
                mu[k][j] -= Rational::from(&x) * shift;
            }
        }
        size_reduce(k, d, basis, gram, mu, r, eta);
    }
}
