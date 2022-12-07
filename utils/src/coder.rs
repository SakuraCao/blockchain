use bincode;
use sha3::{Digest, Sha3_256};
use serde::{Deserialize, Serialize};

pub fn my_serialize<T: ?Sized>(value: &T) -> Vec<u8>
where
    T: Serialize,
{
    let serialized = bincode::serialize(value).unwrap();
    serialized
}

pub fn my_deserialize<'a, T>(bytes: &'a [u8]) -> T
where
    T: Deserialize<'a>,
{
    let deserialized = bincode::deserialize(bytes).unwrap();
    deserialized
}

pub fn get_hash(value: &[u8]) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(value);
    let res = hasher.finalize();
    String::from_utf8_lossy(res)
}

//for test
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use crate::coder::Point;
    use crate::coder::{my_deserialize, my_serialize, get_hash};

    #[test]
    fn coder_works() {
        let point = Point { x: 1, y: 1 };
        let se = my_serialize(&point);
        let de: Point = my_deserialize(&se);
        assert_eq!(de, point);
    }

    #[test]
    fn hash_works() {
        let value = [1, 2];
        let res = get_hash(&value);
        println!("{}", res);
    }
}
