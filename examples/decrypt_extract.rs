use color_eyre::{eyre::Context, Report, Result};
use pelite::pattern;
use pelite::pe32::*;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    if args.len() < 1 {
        Err(Report::msg("Not enough arguments!"))
    } else {
        let file_name = args.next().unwrap();
        let parent = std::path::Path::new(&file_name).parent().unwrap();
        let file_data = std::fs::read(&file_name)
            .wrap_err("can't read file!")
            .unwrap();
        let pe = PeFile::from_bytes(&file_data)
            .wrap_err("invalid PE file!")
            .unwrap();
        assert_eq!(
            pe.file_header().TimeDateStamp,
            0x3FD02712,
            "unknown timestamp!"
        );

        for i in 0..5 {
            const MODEL_FOLDERS: [&str; 5] = ["common", "japanet", "indicator", "ending", "osaka"];

            let mut save = [0u32; 4];
            assert!(
                pe.scanner()
                    .exec(0xE51F8 + i * 4 * 3, pattern!("*{'} u4 *{'}"), &mut save),
                "extract bin file pattern model {}",
                i
            );
            // eprintln!("{:X?}", save);
            let model_path = parent.join("model").join(MODEL_FOLDERS[i as usize]);
            std::fs::create_dir_all(&model_path)?;
            let fname = pe.derva_c_str(save[1]).unwrap().to_str().unwrap();
            let key = save[2];
            let files_rva = save[3];

            let pak = std::fs::read(parent.join(fname))
                .wrap_err("error reading model file!")
                .unwrap();
            let pak = osaka_sim_re::bin::decrpyt(&pak, key)
                .wrap_err("error decrypting model file!")
                .unwrap();

            dump_files(files_rva, pe, &pak, &model_path, ".hgm")?;

            // textures?
            const TEXTURE_DATA: [(&'static str, u32, u32); 5] = [
                ("model/texture00.bin", 0x83D9DB43, 0xE3720),
                ("model/texture01.bin", 0xFE6725D1, 0xE38B8),
                ("model/texture02.bin", 0x75893254, 0xE3E98),
                ("model/texture03.bin", 0x323D47A5, 0xE3EC0),
                ("model/texture04.bin", 0x98D57FFC, 0xE3F38),
            ];
            let (fname, key, files_rva) = TEXTURE_DATA[i as usize];
            let pak = std::fs::read(parent.join(fname))
                .wrap_err("error reading texture file!")
                .unwrap();
            let pak = osaka_sim_re::bin::decrpyt(&pak, key)
                .wrap_err("error decrypting texture file!")
                .unwrap();
            dump_files(files_rva, pe, &pak, &model_path, ".tga")?;
        }

        for i in 0..4 {
            const ANIMATION_FOLDERS: [&str; 4] = ["", "japanet", "indicator", "ending"];

            let mut save = [0u32; 4];
            assert!(
                pe.scanner().exec(
                    0xE51F8 + 16 * 4 + i * 4 * 3,
                    pattern!("*{'} u4 *{'}"),
                    &mut save
                ),
                "extract bin file pattern animation {}",
                i
            );
            // eprintln!("{:X?}", save);
            let animation_path = parent.join("animation").join(ANIMATION_FOLDERS[i as usize]);
            std::fs::create_dir_all(&animation_path)?;
            let fname = pe.derva_c_str(save[1]).unwrap().to_str().unwrap();
            let key = save[2];
            let files_rva = save[3];

            let pak = std::fs::read(parent.join(fname))
                .wrap_err("error reading animation file!")
                .unwrap();
            let pak = osaka_sim_re::bin::decrpyt(&pak, key)
                .wrap_err("error decrypting animation file!")
                .unwrap();

            dump_files(files_rva, pe, &pak, &animation_path, ".hga")?;
        }

        {
            let mut save = [0u32; 4];
            assert!(
                pe.scanner().exec(
                    0xE51F8 + 16 * 4 + 12 * 4,
                    pattern!("*{'} u4 *{'}"),
                    &mut save
                ),
                "extract bin file pattern clipper"
            );
            // eprintln!("{:X?}", save);
            let clipper_path = parent.join("clipper");
            let fname = pe.derva_c_str(save[1]).unwrap().to_str().unwrap();
            let key = save[2];
            let files_rva = save[3];

            let pak = std::fs::read(parent.join(fname))
                .wrap_err("error reading clipper file!")
                .unwrap();
            let pak = osaka_sim_re::bin::decrpyt(&pak, key)
                .wrap_err("error decrypting clipper file!")
                .unwrap();

            dump_files(files_rva, pe, &pak, &clipper_path, ".bmp")?;
        }

        for i in 0..2 {
            let mut save = [0u32; 4];
            assert!(
                pe.scanner().exec(
                    0xE51F8 + 16 * 4 + 12 * 4 + 3 * 4 + 4 + i * 4 * 3,
                    pattern!("*{'} u4 *{'}"),
                    &mut save
                ),
                "extract bin file pattern sound {}",
                i
            );
            let sound_path = parent.join("sound");
            std::fs::create_dir_all(&sound_path)?;
            let fname = pe.derva_c_str(save[1]).unwrap().to_str().unwrap();
            let key = save[2];
            let files_rva = save[3];

            let pak = std::fs::read(parent.join(fname))
                .wrap_err("error reading sound file!")
                .unwrap();
            let pak = osaka_sim_re::bin::decrpyt(&pak, key)
                .wrap_err("error decrypting sound file!")
                .unwrap();

            dump_files(files_rva, pe, &pak, &sound_path, ".wav")?;
        }

        Ok(())
    }
}

fn dump_files(
    files_rva: u32,
    pe: PeFile,
    pak: &[u8],
    model_path: &std::path::PathBuf,
    ext: &str,
) -> Result<(), Report> {
    let mut save = [0u32; 4];
    Ok(for file_rva in (files_rva..).step_by(4 * 3) {
        if !pe
            .scanner()
            .exec(file_rva, pattern!("u4 u4 *{'}"), &mut save)
        {
            break;
        }
        if save[2] == 0 {
            break;
        }
        let start = save[1] as usize;
        let len = save[2] as usize;
        let name = pe.derva_c_str(save[3]).unwrap().to_str().unwrap();

        let file_data = &pak[start..start + len];
        std::fs::write(model_path.join(format!("{name}{ext}")), file_data)?;
    })
}
