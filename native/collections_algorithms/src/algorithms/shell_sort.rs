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

pub fn shell_sort<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let iter: NifListIterator = args[0].decode()?;
    let res: NifResult<Vec<i64>> = iter
        .map(|x: NifTerm| x.decode::<i64>())
        .collect();

    match res {
        Ok(result) => {
            Ok((atoms::ok(), alg_shell_sort(result)).encode(env))
        }
        Err(err) => Err(err)
    }
}

fn alg_shell_sort(arr: Vec<i64>) -> Vec<i64> {
    let mut xs: Vec<i64> = arr.clone();
    let mut gaps: Vec<usize> = [].to_vec();
    let mut g = 0;
    let mut k = 1;
    while g < xs.len() / 3 {
        g = (3_usize.pow(k) - 1) / 2;
        gaps.push(g);
        k += 1;
    }//Pratt, 1971
    gaps.reverse();
    for mut gap in gaps {
        for i in gap..xs.len() {
            let t = xs[i];
            let mut j: usize = i;
            while j >= gap && xs[j - gap] > t {
                xs[j] = xs[j - gap];
                j -= gap;
            }
            xs[j] = t;
        }
    }
    return xs;
}
