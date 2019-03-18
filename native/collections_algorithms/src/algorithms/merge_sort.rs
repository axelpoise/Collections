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

pub fn merge_sort<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let iter: NifListIterator = args[0].decode()?;
    let res: NifResult<Vec<i64>> = iter
        .map(|x: NifTerm| x.decode::<i64>())
        .collect();

    match res {
        Ok(result) => {
            Ok((atoms::ok(), alg_merge_sort(result)).encode(env))
        }
        Err(err) => Err(err)
    }
}


fn alg_merge_sort(arr: Vec<i64>) -> Vec<i64> {
    let mut xs = arr.clone();
    if xs.len() < 2 {
        return xs;
    }
    let length = xs.len() - 1;
    let left = length / 2;
    let right = left + 1;

    let mut ys = &mut xs[0..left + 1].to_vec();
    let mut zs = &mut xs[right..length + 1].to_vec();


    sort(&mut zs);
    sort(&mut ys);
    merge(&mut xs, &mut ys.clone(), &mut zs.clone());

    return xs;
}

fn sort(xs: &mut Vec<i64>) {
    if xs.len() < 2 {
        xs;
    } else {
        let length = xs.len() - 1;
        let left = length / 2;
        let right = left + 1;
        let mut ys = &mut xs[0..left + 1].to_vec();
        let mut zs = &mut xs[right..length + 1].to_vec();
        sort(&mut zs);
        sort(&mut ys);
        merge(xs, &mut ys.clone(), &mut zs.clone());
    }
}

fn merge(xs: &mut Vec<i64>, ys: &mut Vec<i64>, zs: &mut Vec<i64>) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    loop {
        if k > zs.len() - 1 {
            if i > ys.len() - 1 {
                break;
            } else {
                while i < ys.len() {
                    xs[j] = ys[i];
                    j = j + 1;
                    i = i + 1;
                }
               break;
            }
        }
        if i > ys.len() - 1 {
            if k > zs.len() - 1 {
                break; // Unnecessary
            } else {
                while k < zs.len() {
                    xs[j] = zs[k];
                    j = j + 1;
                    k = k + 1;
                }
                break;
            }
        }

        if ys[i] <= zs[k] {
            xs[j] = ys[i];
            i = i + 1;
        } else {
            xs[j] = zs[k];
            k = k + 1;
        }
        j = j + 1;
    }
}

#[test]
fn test_merge() {
    let mut arr1 = [1].to_vec();
    let mut arr2 = [2].to_vec();
    let mut arr3 = [2,1].to_vec();

    merge(&mut arr3, &mut arr1, &mut arr2);
    println!("{:?}", arr3);
    println!("1,2,2,3");
}

#[test]
fn test_sort() {
    let mut arr = [3,2].to_vec();

    sort(&mut arr);
    println!("sorting merge: {:?}", arr)
}