pub mod instruction;
pub(crate) mod register;
pub(crate) mod token;
pub(crate) mod directive;
pub mod program;

pub(crate) trait ValidBitRepr {}
impl ValidBitRepr for Bits<32> {}
impl ValidBitRepr for Bits<6> {}
impl ValidBitRepr for Bits<5> {}

#[derive(Debug, PartialEq, Eq)]
pub enum BitStorage {
    U8(u8),
    U32(u32),
}

/// Represents up to 32 bits of information
#[derive(Debug, PartialEq, Eq)]
#[allow(private_bounds)]
pub struct Bits<const N: usize>
//PERF: replace with its own type if used only in opcode
where
    Self: ValidBitRepr,
{
    data: BitStorage,
}
#[allow(private_bounds)]
impl<const N: usize> Bits<N>
where
    Self: ValidBitRepr,
{
    pub fn new(data: u32) -> Self {
        let data = if N <= 8 {
            assert!(data.leading_zeros() >= 8 - N as u32,);
            BitStorage::U8(data as u8)
        } else if N <= 32 {
            assert!(data.leading_zeros() >= 32 - N as u32);
            BitStorage::U32(data)
        } else {
            panic!("Bits repr must be at most 32");
        };
        Self { data }
    }
    pub fn get(&self) -> u32 {
        match self.data {
            BitStorage::U8(data) => data as u32,
            BitStorage::U32(data) => data,
        }
    }
}
