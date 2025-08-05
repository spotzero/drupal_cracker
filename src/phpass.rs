// https://git.drupalcode.org/project/drupal/-/blob/11.x/core/lib/Drupal/Core/Password/PhpassHashedPasswordBase.php?ref_type=heads

use hmac_sha512::Hash;

pub fn crypt(password: &str, stored_hash: &str) -> String {
  let setting = stored_hash[0..12].to_string();
  let countlog2 = countlog2(setting.as_str());
  let salt = setting[4..12].to_string();

  let iterations = 1 << countlog2;
  let input = salt + password;
  let mut hash = Hash::hash(input);
  let mut count = 0;
  while count < iterations {
    count = count + 1;
    hash = Hash::hash([&hash, password.as_bytes()].concat());
  }

  let mut output = drupal_base64_encode(&hash, 55);
  output = setting + &output;
  output.truncate(55);
  return output;
}

fn countlog2(setting: &str) -> usize {
  let itoa64 = "./0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
  let i = setting.chars().nth(3).unwrap();
  itoa64.find(i).unwrap()
}

pub fn check(password: &str, hash: &str) -> bool {
  let new_hash = crypt(password, hash);
  //println!("{}:\n{}\n{}",password, hash, new_hash);
  if new_hash == hash {
    return true;
  }
  return false;
}

fn drupal_base64_encode(input: &[u8], count: usize) -> String {
    let itoa64: Vec<usize> = "./0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".as_bytes().iter().map(|x| { *x as usize}).collect();
    let mut output = Vec::new();
    let mut i = 0;
    while i <= count {
      let mut value = input[i] as usize;
      i = i + 1;
      output.push(itoa64[(value & 0x3f) as usize]);
      if i < count {
        value = value | (input[i] as usize) << 8;
      }
      output.push(itoa64[((value as usize >> 6) & 0x3f) as usize]);
      if i >= count {
        break;
      }
      i = i + 1;

      if i < count {
        value = value | (input[i] as usize) << 16;
      }
      output.push(itoa64[((value as usize >> 12) & 0x3f) as usize]);
      if i >= count {
        break;
      }
      i = i + 1;
      output.push(itoa64[((value as usize >> 18) & 0x3f) as usize]);
    }
    let output_u8: Vec<u8> = output.iter().map(|x|{return *x as u8}).collect();
    return String::from_utf8_lossy(output_u8.as_slice()).to_string();
  }
