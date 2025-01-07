#![allow(clippy::unused_unit)]
use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use std::fmt::Write;
use pyo3_polars::export::polars_core::utils::CustomIterTools;
use std::ops::{AddAssign,Add,Neg};
use serde::{Deserialize,Serialize};


#[derive(Serialize,Deserialize)]
struct AddCapKwargs {
    cap: i64,
}


#[polars_expr(output_type=String)]
fn pig_latinnify(inputs: &[Series]) -> PolarsResult<Series> {
    let ca: &StringChunked = inputs[0].str()?;
    let out: StringChunked = ca.apply_into_string_amortized(|value: &str, output: &mut String| {
        if let Some(first_char) = value.chars().next() {
            write!(output, "{}{}ay", &value[1..], first_char).unwrap()
        }
    });
    Ok(out.into_series())
}
fn det_sum<Int64>(state: &mut Int64, v: Option<Int64>,cap:Int64) -> Option<Option<Int64>> where
Int64: Add<Output = Int64> + AddAssign  + Copy+ PartialOrd + From<i64> + Neg<Output = Int64>,
{
    match v {
        Some(v) => {
            let new_state = *state+v;
            if new_state > cap || (-new_state) > cap {
                *state = if new_state > cap { cap } else { -cap };
            }else{
                *state += v;
            }
            Some(Some(*state))
        },
        None => Some(None),
    }
}
#[polars_expr(output_type=Int64)]
fn capped_cum_sum(inputs: &[Series],kwargs:AddCapKwargs) -> PolarsResult<Series> {
    let ca = inputs[0].i64()?;
    let init = 0;
    let cap = kwargs.cap;

    let out:Int64Chunked = ca.iter().scan(init,|state,v| det_sum(state,v,cap)).collect_trusted();

    Ok(out.into_series())
}