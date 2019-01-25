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


pub fn bubble_sort<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let iter: NifListIterator = args[0].decode()?;
    let res: NifResult<Vec<i64>> = iter
        .map(|x : NifTerm| x.decode::<i64>())
        .collect();

    match res {
        Ok(result) => {

            Ok((atoms::ok(), alg_bubble_sort(result)).encode(env))
        },
        Err(err) => Err(err)
    }
}

fn alg_bubble_sort(arr: Vec<i64>) ->  Vec<i64> {
    let mut xs: Vec<i64> = arr.clone();
    loop {
        let mut swap: bool = false;
        for i in 0..arr.len()-1 {
            if xs[i] > xs[i+1] {
                swap = true;
                let t= xs[i+1];
                let t_plus_one = xs[i];
                xs[i] = t;
                xs[i+1] = t_plus_one;
            }
        }
        if!swap {break;}
    }
    return xs;
}
