use std::{cell::Cell, collections::HashMap};

use indexmap::IndexMap;
use rustc_hash::FxHashMap;

use crate::model::{Time, id::WeaverailId};

/// ヒープとスタックのメモリ使用量を取得することができる型の特性
pub trait TotalSizable<T> {
    /// スタック上のメモリ使用量を取得する関数
    fn get_stack_memory_size(&self) -> usize {
        std::mem::size_of::<T>()
    }
    /// ヒープ上のメモリ使用量を取得する関数
    fn get_heap_memory_size(&self) -> usize;
    fn get_total_memory_size(&self) -> usize {
        self.get_stack_memory_size() + self.get_heap_memory_size()
    }
}

/// プリミティブ型に対してのボイラープレート
macro_rules! primitive_total_size {
    ($ty: ident) => {
        impl TotalSizable<$ty> for $ty {
            fn get_heap_memory_size(&self) -> usize {
                0
            }
        }
    };
}

primitive_total_size!(bool);
primitive_total_size!(u8);
primitive_total_size!(i8);
primitive_total_size!(u16);
primitive_total_size!(i16);
primitive_total_size!(u32);
primitive_total_size!(i32);
primitive_total_size!(u64);
primitive_total_size!(i64);
primitive_total_size!(u128);
primitive_total_size!(i128);
primitive_total_size!(f32);
primitive_total_size!(f64);
primitive_total_size!(char);
primitive_total_size!(WeaverailId);
primitive_total_size!(Time);

impl TotalSizable<String> for String {
    fn get_heap_memory_size(&self) -> usize {
        self.len()
    }
}

impl<T> TotalSizable<Vec<T>> for Vec<T>
where
    T: TotalSizable<T>,
{
    fn get_heap_memory_size(&self) -> usize {
        self.iter().fold(0, |b, c| b + c.get_total_memory_size())
    }
}

impl<K, V> TotalSizable<HashMap<K, V>> for HashMap<K, V>
where
    K: TotalSizable<K>,
    V: TotalSizable<V>,
{
    fn get_heap_memory_size(&self) -> usize {
        self.iter().fold(0, |b, (k, v)| {
            b + k.get_total_memory_size() + v.get_total_memory_size()
        })
    }
}

impl<K, V> TotalSizable<FxHashMap<K, V>> for FxHashMap<K, V>
where
    K: TotalSizable<K>,
    V: TotalSizable<V>,
{
    fn get_heap_memory_size(&self) -> usize {
        self.iter().fold(0, |b, (k, v)| {
            b + k.get_total_memory_size() + v.get_total_memory_size()
        })
    }
}

impl<K, V> TotalSizable<IndexMap<K, V>> for IndexMap<K, V>
where
    K: TotalSizable<K>,
    V: TotalSizable<V>,
{
    fn get_heap_memory_size(&self) -> usize {
        self.iter().fold(0, |b, (k, v)| {
            b + k.get_total_memory_size() + v.get_total_memory_size()
        })
    }
}


impl<T> TotalSizable<Cell<T>> for Cell<T>
where
    T: TotalSizable<T>,
{
    fn get_heap_memory_size(&self) -> usize {
        unsafe {
            (*self.as_ptr()).get_heap_memory_size()
        }
    }
}
