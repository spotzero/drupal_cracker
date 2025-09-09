
# Drupal Cracker

This project is a very basic password cracker that cracks Drupal 7, 8, 9, 10, and 11 password hashes from a dictionary of passwords.

It's not very fast for testing against a very large dictionary of passwords (like the one included in this project), but it ideal in case you suspect an account is using a trivial password.

## Usage/Examples

This tools requires that a password list be either in the same directory or be provided. Passwords are tested against the hash in the order they appear in the list.  Once password per line in the password list.

Attempt to crack a hash:
```
./drupal_cracker '$2y$10$QczU42cYr1/bjaBJpY08DeV3lqM1MDBjV9obq7Pe75w3NWRf680/a'

```

Attempt to crack a hash with a provided password list:
```
./drupal_cracker -p my_password_list.txt '$2y$10$QczU42cYr1/bjaBJpY08DeV3lqM1MDBjV9obq7Pe75w3NWRf680/a'

```

## Building

You should not need any other dependencies other that the Rust's dev toolchain.

Just run:

```
cargo build -r
```
## License

Distributed under the Apache license, see LICENSE-APACHE for details.


## Contact

David Pascoe-Deslauriers - dpascoed@coldfrontlabs.ca