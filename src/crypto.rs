extern crate crypto;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

// pub struct Crypto {
//     payload: String
// }

// hash with Sha256
pub fn hash(payload:String)-> String{
   let mut sha = Sha256::new();
   sha.input_str(&payload);
  let result= sha.result_str();
  return result.to_string();
   // println!("{:?}", sha.result_str() );
}
