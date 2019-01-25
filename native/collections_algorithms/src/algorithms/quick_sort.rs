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


pub fn quick_sort<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let iter: NifListIterator = args[0].decode()?;
    let res: NifResult<Vec<i64>> = iter
        .map(|x: NifTerm| x.decode::<i64>())
        .collect();

    match res {
        Ok(result) => {
            Ok((atoms::ok(), alg_quick_sort(result)).encode(env))
        }
        Err(err) => Err(err)
    }
}

fn alg_quick_sort(arr: Vec<i64>) -> Vec<i64> {
    let mut xs: Vec<i64> = arr.clone();


    fn sort(high: usize, low: usize) {
        if low < high {
            let p = partition(high, low);
            sort(high, p);
            sort(high + 1, low);
        }
    }

    fn partition(high: usize, low: usize) -> usize {
        let pivot = |xs|[(low + high) / 2];
        let mut i = low - 1;
        let mut j = high + 1;
        loop {
            if |xs|[i] < pivot {
                i += i;
            }
            if |xs|[j] > pivot {
                j -= j;
            }
            if i >= j {
                return j;
            }
            swap(i, j)
        }
    }

    fn swap(i: usize, j: usize) {
        let t1 = |xs|[i];
        let t2 = |xs|[j];
        xs[i] = t2;
        xs[j] = t1;
    }

    sort(0, xs.len() - 1);
    xs
}