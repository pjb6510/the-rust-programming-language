pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
    }
}
