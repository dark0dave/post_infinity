use std::{
    mem::{size_of, ManuallyDrop},
    ptr,
    rc::Rc,
    vec,
};

use crate::common::variable_char_array::VariableCharArray;

pub fn vec_to_u8_slice<T: Sized>(p: &[T]) -> Vec<u8> {
    p.iter()
        .flat_map(|item| to_u8_slice(item))
        .copied()
        .collect()
}

pub fn to_u8_slice<T: Sized>(p: &T) -> &[u8] {
    unsafe { return core::slice::from_raw_parts((p as *const T) as *const u8, size_of::<T>()) }
}

pub fn copy_buff_to_struct<T>(buffer: &[u8], start: usize) -> T {
    let end: usize = start + size_of::<T>();
    if let Some(buff) = buffer.get(start..end) {
        return unsafe { std::ptr::read(buff.as_ptr() as *const _) };
    }
    panic!("Could not extract buffer into struct")
}

pub fn copy_transmute_buff<T>(buffer: &[u8], start: usize, count: usize) -> Vec<T> {
    let end: usize = start + size_of::<T>() * count;
    if let Some(buff) = buffer.get(start..end) {
        let (head, aligned, tail) = unsafe { buff.align_to::<T>() };
        assert!(head.is_empty(), "Data was not aligned");
        assert!(tail.is_empty(), "Data was not aligned");
        assert!(aligned.len() == count, "Data was not aligned");

        let v: Vec<T> = Vec::with_capacity(count);
        let mut v = ManuallyDrop::new(v);
        let ptr: *mut T = v.as_mut_ptr();

        unsafe {
            for (counter, t) in aligned.iter().enumerate() {
                let tmp = ptr::read(t);
                ptr::write(ptr.add(counter), tmp);
            }
            Vec::from_raw_parts(ptr, count, size_of::<T>() * count)
        }
    } else {
        vec![]
    }
}

const CARRAGE_RETURN: u8 = 0xD;
const NEW_LINE: u8 = 0xA;

pub fn dumb_row_parser(buffer: &[u8]) -> Vec<VariableCharArray> {
    let mut acc = vec![];
    let mut pos = 0;
    for (i, x) in buffer.iter().enumerate() {
        if x == &NEW_LINE || x == &CARRAGE_RETURN {
            if pos < i {
                acc.push(VariableCharArray(buffer.get(pos..i).unwrap().into()))
            }
            acc.push(VariableCharArray(Rc::new([32])));
            pos = i;
        }
    }
    if pos < buffer.len() {
        acc.push(VariableCharArray(
            buffer.get(pos..buffer.len()).unwrap().into(),
        ))
    }
    acc
}

// TODO: Fix this absolute trash
pub fn row_parser(buffer: &[u8], row_start: usize) -> (Vec<VariableCharArray>, usize) {
    if let Some(end) = buffer
        .get(row_start..)
        .unwrap_or_default()
        .iter()
        .position(|&byte| byte == CARRAGE_RETURN || byte == NEW_LINE)
    {
        let row_end = row_start + end;
        let row_buff = buffer.get(row_start..row_end).unwrap_or_default();
        let out = row_buff
            .split(|num| num.is_ascii_whitespace())
            .flat_map(|buff| {
                if buff.is_empty() {
                    return None;
                }
                Some(VariableCharArray(buff.into()))
            })
            .collect();

        // Row end to end of line (This should only ever be run twice)
        return (out, row_end + set_to_end_of_line(row_end, buffer));
    }
    (vec![], row_start)
}

fn set_to_end_of_line(row_end: usize, buffer: &[u8]) -> usize {
    match buffer.get(row_end) {
        Some(&CARRAGE_RETURN) => 1 + set_to_end_of_line(row_end + 1, buffer),
        Some(&NEW_LINE) => 1 + set_to_end_of_line(row_end + 1, buffer),
        _ => 0,
    }
}
