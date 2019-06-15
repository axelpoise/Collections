#[macro_use] extern crate rustler;
#[macro_use] extern crate rustler_codegen;
#[macro_use] extern crate lazy_static;

mod algorithms;

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
        ("shell_sort", 1, shell_sort),
        ("quick_sort", 1, quick_sort),
        ("merge_sort", 1, merge_sort),
        ("heap_sort", 1, heap_sort)
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
    algorithms::bubble_sort(env, args)
}


fn insertion_sort<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    algorithms::insertion_sort(env, args)
}

fn selection_sort<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    algorithms::selection_sort(env, args)
}

fn shell_sort<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    algorithms::shell_sort(env, args)
}

fn quick_sort<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    algorithms::quick_sort(env, args)
}

fn merge_sort<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    algorithms::merge_sort(env, args)
}

fn heap_sort<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    algorithms::heap_sort(env, args)
}