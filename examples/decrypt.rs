use color_eyre::eyre::{Report, Result, WrapErr};

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut args = std::env::args().skip(1);
    if args.len() < 2 {
        Err(Report::msg("Not enough arguments!"))
    } else {
        let file_name = args.next().unwrap();
        let key = args.next().unwrap();
        let key = if key.starts_with("0x") {
            u32::from_str_radix(&key, 16)
                .wrap_err("can't parse hex key!")
                .unwrap()
        } else {
            u32::from_str_radix(&key, 10)
                .wrap_err("can't parse dec key!")
                .unwrap()
        };
        let data = std::fs::read(&file_name)
            .wrap_err("can't read file!")
            .unwrap();
        let data = osaka_sim_re::bin::decrpyt(&data, key)
            .wrap_err("error decrypting file!")
            .unwrap();
        std::fs::write(file_name + ".dec", data)
            .wrap_err("error writing decrypted file!")
            .unwrap();
        Ok(())
    }
}
