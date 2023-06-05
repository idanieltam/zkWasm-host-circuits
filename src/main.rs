#![feature(array_zip)]
#![feature(slice_flatten)]
#![feature(int_log)]
pub mod circuits;
pub mod utils;

use clap::{arg, value_parser, App, Arg, ArgMatches};
use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{AssignedCell, Chip, Layouter, SimpleFloorPlanner},
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Fixed},
    poly::Rotation,
};
use std::marker::PhantomData;
use std::{fs::File, io::BufReader, path::PathBuf};

use crate::circuits::{
    bls::Bls381PairChip, bls::Bls381SumChip, bn256::Bn256PairChip, bn256::Bn256SumChip,
    HostOpSelector,
};

use halo2ecc_s::circuit::{
    base_chip::{BaseChip, BaseChipConfig},
    range_chip::{RangeChip, RangeChipConfig},
    select_chip::{SelectChip, SelectChipConfig},
};

use halo2_proofs::dev::MockProver;
use halo2_proofs::pairing::bn256::Fr;