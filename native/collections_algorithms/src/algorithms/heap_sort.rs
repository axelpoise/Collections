use rustler::{NifEnv, NifError, NifTerm, NifResult, NifEncoder};
use rustler::types::list::NifListIterator;
use std::collections::BinaryHeap;
mod atoms {
    rustler_atoms! {
        atom ok;
        atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}



pub fn heap_sort<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let iter: NifListIterator = args[0].decode()?;
    let res: NifResult<Vec<i64>> = iter
        .map(|x : NifTerm| x.decode::<i64>())
        .collect();

    match res {
        Ok(result) => {

            Ok((atoms::ok(), alg_heap_sort(result)).encode(env))
        },
        Err(err) => Err(err)
    }
}

fn alg_heap_sort(arr: Vec<i64>) ->  Vec<i64> {
    return arr;
}

fn build_max_heap (xs: Vec<i64>) {
  let mut bh = BinaryHeap::from(xs);

}

fn sift_down(xs: Vec<i64>) {

}