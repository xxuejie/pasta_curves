use pasta_curves::{arithmetic::SqrtTables, Fp, Fq};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("./data")?;

    let fq_tables: SqrtTables<Fq> = SqrtTables::new(0x116A9E, 1206);
    fs::write("./data/fq.data", fq_tables.dump()?)?;

    let fp_tables: SqrtTables<Fp> = SqrtTables::new(0x11BE, 1098);
    fs::write("./data/fp.data", fp_tables.dump()?)?;

    Ok(())
}
