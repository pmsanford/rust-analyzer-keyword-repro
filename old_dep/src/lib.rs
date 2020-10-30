mod async;

pub use self::async::some_func;

pub fn other_func() -> i32 { 6 }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
