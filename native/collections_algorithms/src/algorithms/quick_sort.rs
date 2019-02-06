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

fn _partition(high: usize, low: usize, xs: &mut Vec<i64>) -> usize {
    let pivot = xs[(low + high) / 2];

    let mut i = low;
    let mut j = high;

        while xs[i] < pivot {
            i = i + 1;
        }
        while xs[j] > pivot {
            j = j - 1;
        }

        if i >= j {
            return j;
        }

        xs.swap(i, j);


    return j;
}

struct Mem<usize> {
    high: usize,
    low: usize,
    pivot: usize,
}

fn alg_quick_sort(arr: Vec<i64>) -> Vec<i64> {
    let mut xs: Vec<i64> = arr.clone();

    let high = xs.len() - 1;
    let low = 0;
    let mut ps: Vec<Mem<usize>> = Vec::new();

    ps.push(Mem {
        high,
        low,
        pivot: _partition(high, low, &mut xs),
    });
    loop {
        let p: Option<Mem<usize>> = ps.pop();

        if p.is_none() {
            return xs;
        }
        let next = p.unwrap();

        if next.low < next.pivot {
            let mem1 = Mem {
                high: next.pivot,
                low: next.low,
                pivot: _partition(next.pivot, next.low, &mut xs),
            };
            ps.push(mem1);
        }

        if next.pivot + 1 < next.high {
            let mem2 = Mem {
                high: next.high,
                low: next.pivot + 1,
                pivot: _partition(next.high, next.pivot + 1, &mut xs),
            };
            ps.push(mem2);
        }
    }
}


#[test]
fn testing () {
    let p = _partition(2, 0, &mut [3,1,2].to_vec());
    println!("{}", p);
}

