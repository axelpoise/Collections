#[macro_use] extern crate rustler;
#[macro_use] extern crate rustler_codegen;
#[macro_use] extern crate lazy_static;

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

rustler_export_nifs! {
    "Elixir.Collections.Algorithms",
    [
        ("add", 2, add),
        ("list", 1, list),
        ("bubble_sort", 1, bubble_sort),
        ("insertion_sort", 1, insertion_sort),
        ("selection_sort", 1, selection_sort),
        ("shell_sort", 1, shell_sort)
    ],
    None
}

fn add<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let num1: i64 = try!(args[0].decode());
    let num2: i64 = try!(args[1].decode());

    Ok((atoms::ok(), num1 + num2).encode(env))
}

fn list<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let iter: NifListIterator = args[0].decode()?;
    let res: NifResult<Vec<i64>> = iter
        .map(|x : NifTerm| x.decode::<i64>())
        .collect();

    match res {
        Ok(result) => Ok((atoms::ok(), result).encode(env)),
        Err(err) => Err(err)
    }
}

fn bubble_sort<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
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


fn insertion_sort<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
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

fn selection_sort<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
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

fn shell_sort<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let iter: NifListIterator = args[0].decode()?;
    let res: NifResult<Vec<i64>> = iter
        .map(|x : NifTerm| x.decode::<i64>())
        .collect();

    match res {
        Ok(result) => {

            Ok((atoms::ok(), alg_shell_sort(result)).encode(env))
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

fn alg_shell_sort(arr: Vec<i64>) -> Vec<i64> {
    let mut xs: Vec<i64> = arr.clone();
    let mut gaps: Vec<usize> = [].to_vec();
    let mut g=0;
    let mut k = 1;
    while g < xs.len()/3 {
        g = (3_usize.pow(k) - 1)/2;
        gaps.push(g);
        k += 1;
    }//Pratt, 1971
    gaps.reverse();
    for mut gap in gaps {

        for i in gap..xs.len() {
            let t = xs[i];
            let mut j : usize = i;
            while j >= gap && xs[j-gap] > t {
                xs[j] = xs[j - gap];
                j -= gap;
            }
            xs[j] = t;
        }
    }
    return xs;
}
