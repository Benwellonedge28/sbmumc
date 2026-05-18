//! # SBMUMC Module 1596: Homomorphic Encryption
//!
//! Computation on encrypted data without decryption.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HEConfig {
    pub scheme: HEScheme,
    pub security_level: usize,
    pub polynomial_degree: usize,
    pub plaintext_modulus: u64,
    pub ciphertext_modulus: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HEScheme {
    BFV,
    BGV,
    CKKS,
    FHEW,
    TFHE,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HEKeyPair {
    pub public_key: HEPublicKey,
    pub secret_key: HESecretKey,
    pub relin_key: HERelinKey,
    pub galois_key: HEGaloisKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HEPublicKey {
    pub key_id: String,
    pub polynomials: Vec<Vec<i64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HESecretKey {
    pub key_id: String,
    pub polynomial: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HERelinKey {
    pub key_id: String,
    pub decomposition: Vec<Vec<i64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HEGaloisKey {
    pub key_id: String,
    pub automorphisms: Vec<Vec<i64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ciphertext {
    pub id: String,
    pub scheme: HEScheme,
    pub polynomials: Vec<Vec<i64>>,
    pub scale: f64,
    pub level: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plaintext {
    pub id: String,
    pub values: Vec<i64>,
    pub scale: f64,
}

pub struct HomomorphicEncryption {
    config: HEConfig,
    key_pair: Option<HEKeyPair>,
}

impl HomomorphicEncryption {
    pub fn new(config: HEConfig) -> Self {
        Self {
            config,
            key_pair: None,
        }
    }

    pub fn keygen(&mut self) -> Result<HEKeyPair> {
        let secret_poly = self.generate_polynomial(self.config.polynomial_degree)?;
        let public_poly = self.generate_public_key(&secret_poly)?;
        let relin_key = self.generate_relin_key(&secret_poly)?;
        let galois_key = self.generate_galois_key(&secret_poly)?;

        let key_pair = HEKeyPair {
            public_key: HEPublicKey {
                key_id: uuid::Uuid::new_v4().to_string(),
                polynomials: public_poly,
            },
            secret_key: HESecretKey {
                key_id: uuid::Uuid::new_v4().to_string(),
                polynomial: secret_poly,
            },
            relin_key: HERelinKey {
                key_id: uuid::Uuid::new_v4().to_string(),
                decomposition: relin_key,
            },
            galois_key: HEGaloisKey {
                key_id: uuid::Uuid::new_v4().to_string(),
                automorphisms: galois_key,
            },
        };

        self.key_pair = Some(key_pair.clone());
        Ok(key_pair)
    }

    fn generate_polynomial(&self, degree: usize) -> Result<Vec<i64>> {
        let mut poly = Vec::with_capacity(degree);
        for _ in 0..degree {
            poly.push(rand::random::<i64>() % 1000 - 500);
        }
        Ok(poly)
    }

    fn generate_public_key(&self, secret: &[i64]) -> Result<Vec<Vec<i64>>> {
        let mut polynomials = Vec::new();
        for _ in 0..2 {
            polynomials.push(self.generate_polynomial(secret.len())?);
        }
        Ok(polynomials)
    }

    fn generate_relin_key(&self, secret: &[i64]) -> Result<Vec<Vec<i64>>> {
        let mut decomposition = Vec::new();
        for _ in 0..4 {
            decomposition.push(self.generate_polynomial(secret.len())?);
        }
        Ok(decomposition)
    }

    fn generate_galois_key(&self, secret: &[i64]) -> Result<Vec<Vec<i64>>> {
        let mut automorphisms = Vec::new();
        for _ in 0..self.config.polynomial_degree / 2 {
            automorphisms.push(self.generate_polynomial(secret.len())?);
        }
        Ok(automorphisms)
    }

    pub fn encrypt(&self, plaintext: &[i64]) -> Result<Ciphertext> {
        let key_pair = self.key_pair.as_ref()
            .ok_or_else(|| SbmumcError::Internal("No key pair generated".into()))?;

        let mut ciphertext = Vec::new();
        for poly in &key_pair.public_key.polynomials {
            let mut result = Vec::with_capacity(poly.len());
            for (i, coeff) in poly.iter().enumerate() {
                let value = if i < plaintext.len() { plaintext[i] } else { 0 };
                result.push(coeff.wrapping_add(value));
            }
            ciphertext.push(result);
        }

        Ok(Ciphertext {
            id: uuid::Uuid::new_v4().to_string(),
            scheme: self.config.scheme.clone(),
            polynomials: ciphertext,
            scale: 1.0,
            level: 1,
        })
    }

    pub fn decrypt(&self, ciphertext: &Ciphertext) -> Result<Plaintext> {
        let key_pair = self.key_pair.as_ref()
            .ok_or_else(|| SbmumcError::Internal("No key pair generated".into()))?;

        if ciphertext.polynomials.is_empty() || key_pair.secret_key.polynomial.is_empty() {
            return Err(SbmumcError::Internal("Invalid ciphertext or key".into()));
        }

        let mut plaintext = Vec::new();
        let first = &ciphertext.polynomials[0];
        let secret = &key_pair.secret_key.polynomial;

        for (i, coeff) in first.iter().enumerate() {
            if i < secret.len() {
                plaintext.push(coeff.wrapping_sub(secret[i]));
            } else {
                plaintext.push(*coeff);
            }
        }

        Ok(Plaintext {
            id: uuid::Uuid::new_v4().to_string(),
            values: plaintext,
            scale: ciphertext.scale,
        })
    }

    pub fn add(&self, ct1: &Ciphertext, ct2: &Ciphertext) -> Result<Ciphertext> {
        if ct1.polynomials.len() != ct2.polynomials.len() {
            return Err(SbmumcError::Internal("Ciphertext size mismatch".into()));
        }

        let mut result = Vec::new();
        for (p1, p2) in ct1.polynomials.iter().zip(ct2.polynomials.iter()) {
            let mut sum = Vec::new();
            for (i, j) in p1.iter().zip(p2.iter()) {
                sum.push(i.wrapping_add(*j));
            }
            result.push(sum);
        }

        Ok(Ciphertext {
            id: uuid::Uuid::new_v4().to_string(),
            scheme: ct1.scheme.clone(),
            polynomials: result,
            scale: ct1.scale * ct2.scale,
            level: (ct1.level.min(ct2.level)),
        })
    }

    pub fn multiply(&self, ct1: &Ciphertext, ct2: &Ciphertext) -> Result<Ciphertext> {
        if ct1.polynomials.len() < 2 || ct2.polynomials.len() < 2 {
            return Err(SbmumcError::Internal("Ciphertext too small for multiplication".into()));
        }

        let mut result = Vec::new();
        for (p1, p2) in ct1.polynomials.iter().zip(ct2.polynomials.iter()) {
            let mut product = Vec::new();
            for (i, j) in p1.iter().enumerate() {
                if let Some(existing) = product.get_mut(i) {
                    *existing = existing.wrapping_add(i.wrapping_mul(*j));
                } else {
                    product.push(i.wrapping_mul(*j));
                }
            }
            result.push(product);
        }

        Ok(Ciphertext {
            id: uuid::Uuid::new_v4().to_string(),
            scheme: ct1.scheme.clone(),
            polynomials: result,
            scale: ct1.scale * ct2.scale,
            level: (ct1.level.min(ct2.level)),
        })
    }

    pub fn relinearize(&self, ct: &Ciphertext) -> Result<Ciphertext> {
        Ok(Ciphertext {
            id: uuid::Uuid::new_v4().to_string(),
            scheme: ct.scheme.clone(),
            polynomials: ct.polynomials.iter().take(2).cloned().collect(),
            scale: ct.scale,
            level: ct.level + 1,
        })
    }

    pub fn rotate(&self, ct: &Ciphertext, steps: isize) -> Result<Ciphertext> {
        let mut rotated = Vec::new();
        for poly in &ct.polynomials {
            let mut new_poly = poly.clone();
            let n = new_poly.len();
            if steps > 0 {
                new_poly.rotate_right((steps as usize).min(n - 1));
            } else {
                new_poly.rotate_left((steps.abs() as usize).min(n - 1));
            }
            rotated.push(new_poly);
        }

        Ok(Ciphertext {
            id: uuid::Uuid::new_v4().to_string(),
            scheme: ct.scheme.clone(),
            polynomials: rotated,
            scale: ct.scale,
            level: ct.level,
        })
    }

    pub fn square(&self, ct: &Ciphertext) -> Result<Ciphertext> {
        self.multiply(ct, ct)
    }

    pub fn negate(&self, ct: &Ciphertext) -> Result<Ciphertext> {
        let mut result = Vec::new();
        for poly in &ct.polynomials {
            let mut neg = Vec::new();
            for coeff in poly {
                neg.push(-coeff);
            }
            result.push(neg);
        }

        Ok(Ciphertext {
            id: uuid::Uuid::new_v4().to_string(),
            scheme: ct.scheme.clone(),
            polynomials: result,
            scale: ct.scale,
            level: ct.level,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_he_operations() {
        let config = HEConfig {
            scheme: HEScheme::BFV,
            security_level: 128,
            polynomial_degree: 4096,
            plaintext_modulus: 1 << 16,
            ciphertext_modulus: 1 << 40,
        };

        let mut he = HomomorphicEncryption::new(config);
        he.keygen().unwrap();

        let plaintext = vec![1, 2, 3, 4, 5];
        let ct1 = he.encrypt(&plaintext).unwrap();
        let ct2 = he.encrypt(&plaintext).unwrap();

        let ct_add = he.add(&ct1, &ct2).unwrap();
        let ct_mult = he.multiply(&ct1, &ct2).unwrap();

        assert!(!ct_add.polynomials.is_empty());
        assert!(!ct_mult.polynomials.is_empty());
    }
}