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
    let p = (low + high) / 2;
    if p == low {
        if xs[high]<xs[low] {
            xs.swap(high, low)
        }
        return p;
    }


    let pivot = xs[p];
    xs.swap(high, p);


    loop {
        let mut i = low;
        let mut j = high - 1;

        while xs[i] < pivot {
            i = i + 1;
        }
        while  j > 0 && xs[j] > pivot {
            j = j - 1;
        }

        if i >= j {
            xs.swap(j, high);
            return j;
        }

        xs.swap(i, j);

    }
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
        if next.high > next.low {
            let piv1 = _partition(next.pivot, next.low, &mut xs);
            let piv2 = _partition(next.high, next.pivot + 1, &mut xs);

            let mem1 = Mem {
                high: next.pivot,
                low: next.low,
                pivot: piv1,
            };
            ps.push(mem1);


            let mem2 = Mem {
                high: next.high,
                low: next.pivot + 1,
                pivot: piv2,
            };
            ps.push(mem2);
        }
    }
}


#[test]
fn testing() {
    let mut xs = [1,2,1].to_vec();
    let p = _partition(2, 1, &mut xs);
    println!("{}", p);
    println!("{:?}", xs);
}


