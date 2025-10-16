// Make your main code modules public (so tests and external crates can see them)
pub mod core_structs;

// Only compiled when running `cargo test`
#[cfg(test)]
mod tests {
    mod worker_tests;
    mod base_tests;
}
