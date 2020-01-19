#[cfg(test)]
mod tests {
    use streaming_iterator::{convert, StreamingIterator};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn streaming_iterator() {
        let v = vec![1, 2, 3];
        let mut vc = convert(v);
        while let Some(score) = vc.next() {
            println!("The score is: {}", score);
        }
    }
}
