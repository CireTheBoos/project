mod index_of;
mod range_of;
mod copy_within_memory;

pub use index_of::IndexOf;
pub use range_of::RangeOf;
pub use copy_within_memory::copy_within_memory_nonoverlapping;

#[cfg(test)]
mod tests {
    use super::RangeOf;

    #[allow(unused)]
    struct Test(u32);

    #[test]
    fn test() {
        let points_range: RangeOf<Test> = RangeOf::new(2, 4);
        assert_eq!(points_range.byte_offset(), 2 * 4);
        assert_eq!(points_range.byte_size(), 4 * 4);

        println!("{points_range}");

        let bytes_range: RangeOf = RangeOf::new(5, 3);
        assert_eq!(bytes_range.byte_offset(), 5);
        assert_eq!(bytes_range.byte_size(), 3);

        assert_eq!(points_range.to_std_range(), (2..6));
        assert_eq!(points_range.as_range_of_bytes().to_std_range(), (8..24));
    }
}
