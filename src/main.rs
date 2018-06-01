extern crate grouille;

fn main() -> Result<(), std::io::Error> {
    grouille::slice::slice("test_files/cordoba.stl", 0.3)?;
    Ok(())
}
