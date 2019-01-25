use rustler::{NifEnv, NifError, NifTerm, NifResult, NifEncoder};
use rustler::types::list::NifListIterator;

mod atoms {
    rustler_atoms! {
        atom ok;
        atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

pub fn insertion_sort<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let iter: NifListIterator = args[0].decode()?;
    let res: NifResult<Vec<i64>> = iter
        .map(|x : NifTerm| x.decode::<i64>())
        .collect();

    match res {
        Ok(result) => {

            Ok((atoms::ok(), alg_insertion_sort(result)).encode(env))
        },
        Err(err) => Err(err)
    }
}


fn alg_insertion_sort(arr: Vec<i64>) ->  Vec<i64> {
    let mut xs: Vec<i64> = arr.clone();
    let mut i = 1;
    while i < arr.len() {
        let mut j = i;
        while j > 0 && xs[j-1] > xs[j] {
            let t= xs[j-1];
            let t_min_one = xs[j];
            xs[j] = t;
            xs[j-1] = t_min_one;
            j = j - 1;
        }
        i = i + 1;
    }
    return xs;

}