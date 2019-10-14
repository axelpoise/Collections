use rustler::{NifEnv, NifError, NifTerm, NifResult, NifEncoder};
use rustler::types::list::NifListIterator;
use std::collections::BinaryHeap;
use rustler::NifEncoder;
mod atoms {
    rustler_atoms! {
        atom ok;
        atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

pub fn heapify() {

}

pub fn add_to_heap(arr:Vec<i64>)  {

}

pub fn next_from_heap() -> i64 {
    return 0;
}

pub fn create_heap<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let iter: NifListIterator = args[0].decode()?;
    let res: NifResult<Vec<i64>> = iter
    .map(|x : NifTerm| x.decode::<i64>())
    .collect();
    let mut heap = BinaryHeap::new();
    heap.push(1);
    heap.push(2);


    match res {
    Ok(result) => {

        Ok((atoms::ok(), heap.data ).encode(env))
    },
    Err(err) => Err(err)
    }
}


fn dirty_encode() {

}

