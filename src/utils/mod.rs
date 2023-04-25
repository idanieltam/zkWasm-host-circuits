pub mod test;

use halo2_proofs::arithmetic::BaseExt;
use halo2_proofs::arithmetic::FieldExt;
use num_bigint::BigUint;
use halo2_proofs::circuit::AssignedCell;

pub fn field_to_bn<F: BaseExt>(f: &F) -> BigUint {
    let mut bytes: Vec<u8> = Vec::new();
    f.write(&mut bytes).unwrap();
    BigUint::from_bytes_le(&bytes[..])
}

pub fn bn_to_field<F: BaseExt>(bn: &BigUint) -> F {
    let mut bytes = bn.to_bytes_le();
    bytes.resize(48, 0);
    let mut bytes = &bytes[..];
    F::read(&mut bytes).unwrap()
}


pub fn field_to_u32<F: FieldExt>(f: &F) -> u32 {
    let mut bytes: Vec<u8> = Vec::new();
    f.write(&mut bytes).unwrap();
    u32::from_le_bytes(bytes[0..4].try_into().unwrap())
}

pub fn field_to_u64<F: FieldExt>(f: &F) -> u64 {
    let mut bytes: Vec<u8> = Vec::new();
    f.write(&mut bytes).unwrap();
    u64::from_le_bytes(bytes[0..8].try_into().unwrap())
}


pub fn u32_to_limbs<F: FieldExt>(v: u32) -> [F; 4] {
    let mut rem = v;
    let mut r = vec![];
    for _ in 0..4 {
        r.append(&mut vec![F::from((rem % 256) as u64)]);
        rem = rem/256;
    }
    r.try_into().unwrap()
}

pub fn cell_to_u32<F: FieldExt>(cell: &AssignedCell<F, F>) -> u32 {
    cell.value().map_or(0, |x| field_to_u32(x))
}

pub fn cell_to_limbs<F: FieldExt>(cell: &AssignedCell<F, F>) -> [F; 4] {
    let a = cell_to_u32(cell);
    u32_to_limbs(a)
}


#[macro_export]
macro_rules! curr {
    ($meta: expr, $x: expr) => {
        $meta.query_advice($x, halo2_proofs::poly::Rotation::cur())
    };
}

#[macro_export]
macro_rules! prev {
    ($meta: expr, $x: expr) => {
        $meta.query_advice($x, halo2_proofs::poly::Rotation::prev())
    };
}

#[macro_export]
macro_rules! next {
    ($meta: expr, $x: expr) => {
        $meta.query_advice($x, halo2_proofs::poly::Rotation::next())
    };
}

#[macro_export]
macro_rules! nextn {
    ($meta: expr, $x: expr, $n:expr) => {
        $meta.query_advice($x, halo2_proofs::poly::Rotation($n))
    };
}

#[macro_export]
macro_rules! fixed_curr {
    ($meta: expr, $x: expr) => {
        $meta.query_fixed($x, halo2_proofs::poly::Rotation::cur())
    };
}

#[macro_export]
macro_rules! instance_curr {
    ($meta: expr, $x: expr) => {
        $meta.query_instance($x, halo2_proofs::poly::Rotation::cur())
    };
}

#[macro_export]
macro_rules! fixed_prev {
    ($meta: expr, $x: expr) => {
        $meta.query_fixed($x, halo2_proofs::poly::Rotation::prev())
    };
}

#[macro_export]
macro_rules! fixed_next {
    ($meta: expr, $x: expr) => {
        $meta.query_fixed($x, halo2_proofs::poly::Rotation::next())
    };
}

#[macro_export]
macro_rules! constant_from {
    ($x: expr) => {
        halo2_proofs::plonk::Expression::Constant(F::from($x as u64))
    };
}

#[macro_export]
macro_rules! constant_from_bn {
    ($x: expr) => {
        halo2_proofs::plonk::Expression::Constant(bn_to_field($x))
    };
}

#[macro_export]
macro_rules! constant {
    ($x: expr) => {
        halo2_proofs::plonk::Expression::Constant($x)
    };
}


#[macro_export]
macro_rules! item_count {
    () => {0usize};
    ($cut:tt nil $($tail:tt)*) => {1usize + item_count!($($tail)*)};
    ($cut:tt $name:tt $($tail:tt)*) => {1usize + item_count!($($tail)*)};
}

#[macro_export]
macro_rules! table_item {
    ($row:expr, $col:expr, ) => {};
    ($row:expr, $col:expr, $cut:tt nil $($tail:tt)*) => {
        table_item!($row, $col, $($tail)*);
    };
    ($row:expr, $col:expr, $cut:tt $name:tt $($tail:tt)*) => {
        fn $name() -> Self {
            let index = $row * $col - 1usize - (item_count!($($tail)*));
            GateCell {
                cell: [Self::typ(index), Self::col(index), Self::row(index)],
                name: String::from(stringify!($name)),
            }
        }
        table_item!($row, $col, $($tail)*);
    };
}

#[macro_export]
macro_rules! customized_curcuits {
    ($name:ident, $row:expr, $col:expr, $adv:expr, $fix:expr, $sel:expr, $($item:tt)* ) => {
        struct GateCell {
            cell: [usize;3],
            name: String,
        }

        impl GateCell {
            fn typ(index: usize) -> usize {
                let x = index % $col;
                if x < $adv {
                    0
                } else if x < $adv + $fix {
                    1
                } else {
                    2
                }
            }

            fn col(index: usize) -> usize {
                let x = index % $col;
                if x < $adv {
                    x
                } else if x < $adv + $fix {
                    x - $adv
                } else {
                    x - $adv - $fix
                }
            }

            fn row(index: usize) -> usize {
                index / $col
            }

            table_item!($row, $col, $($item)*);
        }

        // #[derive(Clone, Debug)]
        // pub struct $name {
        //     witness: [Column<Advice>; $adv],
        //     selector: [Selector; $sel],
        //     fixed: [Column<Fixed>; $fix],
        // }
    };
}