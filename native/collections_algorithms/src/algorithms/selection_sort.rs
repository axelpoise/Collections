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



pub fn selection_sort<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let iter: NifListIterator = args[0].decode()?;
    let res: NifResult<Vec<i64>> = iter
        .map(|x : NifTerm| x.decode::<i64>())
        .collect();

    match res {
        Ok(result) => {

            Ok((atoms::ok(), alg_selection_sort(result)).encode(env))
        },
        Err(err) => Err(err)
    }
}

fn alg_selection_sort(arr: Vec<i64>) ->  Vec<i64> {
    let mut xs: Vec<i64> = arr.clone();
    let mut ys: Vec<i64> = [].to_vec();
    loop {
        let mut t = xs[0];
        let mut j = 0;
        for i in 1..xs.len()  {
            if xs[i] < xs[i-1] {
                t = xs[i];
                j = i;
            }
        }
        xs.remove(j);
        ys.push(t);
        if xs.len() == 0 {
            break;
        }
    }
    return ys;
}