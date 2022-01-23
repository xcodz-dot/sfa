# 1.1.0
* Fixed major bug with `sfa::encode` because of usage of `std::io::Write::write`.
  Now replaced with `std::io::Write::write_all`. (Thanks to `cargo clippy`).
* Renamed `sfa::decode_from_buffer` to `sfa::decode_from_reader` and added function documentation.
* Added documentation for `sfa::encode`.
* Added documentation for `sfa::decode`.
* Added documentation for `sfa::Error`.

# 1.0.0
* Added `sfa::encode`.
* Added `sfa::decode`.
* Added `sfa::decode_from_buffer`.
* Added `sfa::Error`.
* Added module level documentation.