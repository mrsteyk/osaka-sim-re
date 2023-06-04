use std::io::prelude::*;

use color_eyre::{eyre::Context, Report, Result};
use osaka_sim_re::hg::{Block, PTEnum, VertexFeatures};

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut args = std::env::args().skip(1);
    if args.len() < 1 {
        Err(Report::msg("Not enough arguments!"))
    } else {
        let file_name = args.next().unwrap();
        let data = std::fs::read(&file_name)
            .wrap_err("can't read file!")
            .unwrap();
        let blocks = osaka_sim_re::hg::read_blocks(&data);
        println!("{:#?}", blocks);
        // writes obj files?
        for block in blocks {
            if let Block::Geometry(g) = block {
                let vertex_count = g.vertex_num.unwrap();
                let vertex_stride = g.vertex_size.unwrap();
                let vertex_data = g.vertex_data.unwrap();

                let out = std::fs::File::create(file_name.clone() + "_" + g.name + ".obj").unwrap();
                let mut f = std::io::BufWriter::new(out);
                #[allow(unused_assignments)]
                for i in 0..vertex_count as usize {
                    let mut c = vertex_stride as usize * i;
                    if g.vertex_bitmask.contains(VertexFeatures::Position) {
                        let xyz = bytemuck::pod_read_unaligned::<[f32; 3]>(&vertex_data[c..c + 12]);
                        write!(f, "v {} {} {}\n", xyz[0], xyz[1], xyz[2])?;
                        c += 12;
                    }
                    if g.vertex_bitmask.contains(VertexFeatures::Normal) {
                        let norm =
                            bytemuck::pod_read_unaligned::<[f32; 3]>(&vertex_data[c..c + 12]);
                        write!(f, "vn {} {} {}\n", norm[0], norm[1], norm[2])?;
                        c += 12;
                    }
                    // if g.vertex_bitmask.contains(VertexFeatures::Tangent | VertexFeatures::Binormal) {
                    //     todo!("{}", g.vertex_bitmask)
                    // }
                    // if g.vertex_bitmask.contains(VertexFeatures::Color0) {
                    //     c += 12;
                    // }
                    // if g.vertex_bitmask.contains(VertexFeatures::Color1) {
                    //     c += 12;
                    // }
                }
                if g.idk.len() > 1 {
                    todo!()
                }
                let idxs = &g.idk[0];
                if idxs.typ != PTEnum::TriangleStrip {
                    todo!("{:?}", idxs.typ);
                }
                write!(f, "\n")?;
                for i in 0..idxs.words.len() - 2 {
                    if i & 1 != 0 {
                        write!(
                            f,
                            "f {} {} {}\n",
                            idxs.words[i + 0] + 1,
                            idxs.words[i + 1] + 1,
                            idxs.words[i + 2] + 1
                        )?;
                    } else {
                        write!(
                            f,
                            "f {} {} {}\n",
                            idxs.words[i + 0] + 1,
                            idxs.words[i + 2] + 1,
                            idxs.words[i + 1] + 1
                        )?;
                    }
                }
            }
        }
        Ok(())
    }
}
