use thiserror::Error;

pub mod bin {
    use super::*;

    #[derive(Error, Debug)]
    pub enum DecryptError {
        #[error("data too small, should be at least 4 bytes")]
        TooSmall(usize),
    }

    pub fn decrpyt(src: &[u8], key: u32) -> Result<Vec<u8>, DecryptError> {
        if src.len() < 4 {
            // This should never happen???
            Err(DecryptError::TooSmall(src.len()))
        } else {
            let key = key.to_le_bytes();
            Ok(src
                .iter()
                .zip(key.iter().cycle())
                .map(|(&x, &y)| x ^ y)
                .collect())
        }
    }

    // I will not make useless file extraction because it will make this code dependant on pelite!
}

pub mod hg {
    use super::*;
    use bitmask::bitmask;
    use std::str::Utf8Error;

    #[derive(Debug)]
    pub struct TRS3d {
        pub pos: [f32; 3],
        pub rot: [f32; 3],
        pub scale: [f32; 3],
    }

    #[derive(Debug, Eq, PartialEq)]
    pub enum PTEnum {
        // TODO: is start at 0?
        TriangleList = 0,
        TriangleStrip = 1,
        TriangleFan = 2,
        UNK,
    }

    impl From<u32> for PTEnum {
        fn from(value: u32) -> Self {
            match value {
                0 => Self::TriangleList,
                1 => Self::TriangleStrip,
                2 => Self::TriangleFan,
                _ => Self::UNK,
            }
        }
    }

    #[derive(Debug)]
    pub struct GeometryBlockInner {
        pub typ: PTEnum,
        pub words: Vec<u16>,
    }

    bitmask! {
        // #[derive(Debug)]
        pub mask VertexMask: u32
        where flags VertexFeatures {
            Position        = 1<<0,
            Normal          = 1<<1,
            Tangent         = 1<<2,
            Binormal        = 1<<3,

            Color0          = 1<<4,
            Color1          = 1<<5,

            Weight0         = 1<<6,
            Weight1         = 1<<7,
            Weight2         = 1<<8,
            Weight3         = 1<<9,
            WeightIndicies  = 1<<10,

            TexCoordinate0  = 1<<11,
            TexCoordinate1  = 1<<12,
            TexCoordinate2  = 1<<13,
            TexCoordinate3  = 1<<14,
            TexCoordinate4  = 1<<15,
            TexCoordinate5  = 1<<16,
            TexCoordinate6  = 1<<17,
            TexCoordinate7  = 1<<18,
        }
    }

    impl std::fmt::Display for VertexMask {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut first = true;
            // TODO: replace this ugly shit with a macro
            if *(*self & VertexFeatures::Position) != 0 {
                if !first {
                    write!(f, " | ")?;
                }
                write!(f, "VertexFeatures::Position")?;
                first = false;
            }
            if *(*self & VertexFeatures::Normal) != 0 {
                if !first {
                    write!(f, " | ")?;
                }
                write!(f, "VertexFeatures::Normal")?;
                first = false;
            }
            if *(*self & VertexFeatures::Tangent) != 0 {
                if !first {
                    write!(f, " | ")?;
                }
                write!(f, "VertexFeatures::Tangent")?;
                first = false;
            }
            if *(*self & VertexFeatures::Binormal) != 0 {
                if !first {
                    write!(f, " | ")?;
                }
                write!(f, "VertexFeatures::Binormal")?;
                first = false;
            }
            if *(*self & VertexFeatures::Color0) != 0 {
                if !first {
                    write!(f, " | ")?;
                }
                write!(f, "VertexFeatures::Color0")?;
                first = false;
            }
            if *(*self & VertexFeatures::Color1) != 0 {
                if !first {
                    write!(f, " | ")?;
                }
                write!(f, "VertexFeatures::Color1")?;
                first = false;
            }
            if *(*self & VertexFeatures::Weight0) != 0 {
                if !first {
                    write!(f, " | ")?;
                }
                write!(f, "VertexFeatures::Weight0")?;
                first = false;
            }
            if *(*self & VertexFeatures::Weight1) != 0 {
                if !first {
                    write!(f, " | ")?;
                }
                write!(f, "VertexFeatures::Weight1")?;
                first = false;
            }
            if *(*self & VertexFeatures::Weight2) != 0 {
                if !first {
                    write!(f, " | ")?;
                }
                write!(f, "VertexFeatures::Weight2")?;
                first = false;
            }
            if *(*self & VertexFeatures::Weight3) != 0 {
                if !first {
                    write!(f, " | ")?;
                }
                write!(f, "VertexFeatures::Weight3")?;
                first = false;
            }
            if *(*self & VertexFeatures::WeightIndicies) != 0 {
                if !first {
                    write!(f, " | ")?;
                }
                write!(f, "VertexFeatures::WeightIndicies")?;
                first = false;
            }
            if *(*self & VertexFeatures::TexCoordinate0) != 0 {
                if !first {
                    write!(f, " | ")?;
                }
                write!(f, "VertexFeatures::TexCoordinate0")?;
                first = false;
            }
            if *(*self & VertexFeatures::TexCoordinate1) != 0 {
                if !first {
                    write!(f, " | ")?;
                }
                write!(f, "VertexFeatures::TexCoordinate1")?;
                first = false;
            }
            if *(*self & VertexFeatures::TexCoordinate2) != 0 {
                if !first {
                    write!(f, " | ")?;
                }
                write!(f, "VertexFeatures::TexCoordinate2")?;
                first = false;
            }
            if *(*self & VertexFeatures::TexCoordinate3) != 0 {
                if !first {
                    write!(f, " | ")?;
                }
                write!(f, "VertexFeatures::TexCoordinate3")?;
                first = false;
            }
            if *(*self & VertexFeatures::TexCoordinate4) != 0 {
                if !first {
                    write!(f, " | ")?;
                }
                write!(f, "VertexFeatures::TexCoordinate4")?;
                first = false;
            }
            if *(*self & VertexFeatures::TexCoordinate5) != 0 {
                if !first {
                    write!(f, " | ")?;
                }
                write!(f, "VertexFeatures::TexCoordinate5")?;
                first = false;
            }
            if *(*self & VertexFeatures::TexCoordinate6) != 0 {
                if !first {
                    write!(f, " | ")?;
                }
                write!(f, "VertexFeatures::TexCoordinate6")?;
                first = false;
            }
            if *(*self & VertexFeatures::TexCoordinate7) != 0 {
                if !first {
                    write!(f, " | ")?;
                }
                write!(f, "VertexFeatures::TexCoordinate7")?;
                // first = false;
            }
            if self.is_none() {
                write!(f, "()")?;
            }
            Ok(())
        }
    }

    impl std::fmt::Debug for VertexMask {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self)
        }
    }

    #[derive(Debug)]
    pub struct GeometryBlock<'a> {
        pub name: &'a str,
        pub coords: [f32; 4],
        pub bool4: bool,
        pub vertex_bitmask: VertexMask,
        pub bool6: bool,

        pub idk: Vec<GeometryBlockInner>,

        pub vertex_num: Option<u32>,
        pub vertex_size: Option<usize>,
        pub vertex_data: Option<&'a [u8]>,
    }

    #[derive(Debug)]
    pub struct TransformBlock<'a> {
        pub name: &'a str,
        pub idk: u32,
        // T R S
        pub coords: TRS3d, //[[f32; 3]; 3],
                           // pub coords1: [f32; 4],
                           // pub coords2: [f32; 3],
                           // pub coords3: [f32; 3],
    }

    #[derive(Debug)]
    pub struct BoneBlock<'a> {
        pub name: &'a str,
        pub idk: u32,
        pub coords: [[f32; 3]; 3],

        // X = 1, Y = 0
        // 1 4 7 Y
        // 2 5 8 Y
        // 3 6 9 Y
        // Y Y Y X
        pub matrix: [[f32; 4]; 4],
    }

    #[derive(Debug)]
    pub enum Block<'a> {
        Geometry(GeometryBlock<'a>),
        Shader,
        Shape,
        Texture,
        Transform(TransformBlock<'a>),
        Animator,
        AnimationData,
        AnimationSet,
        Hierarchy,
        Bone(BoneBlock<'a>),
    }

    fn read_str<'a>(src: &'a [u8]) -> Result<(&'a str, usize), Utf8Error> {
        let mut size = 0;
        loop {
            if src[size] == 0 {
                break;
            }
            size += 1;
        }
        let skip = 4 * (size / 4) + 4;
        Ok((std::str::from_utf8(&src[..size])?, skip))
    }

    pub fn read_blocks<'a>(src: &'a [u8]) -> Vec<Block<'a>> {
        // if src.len() < 8
        let mut ret = Vec::new();
        let mut cursor = src;
        loop {
            if cursor.len() < 8 {
                break;
            }

            let typ = bytemuck::pod_read_unaligned::<u32>(&cursor[..4]);
            let size = bytemuck::pod_read_unaligned::<u32>(&cursor[4..8]) as usize;
            cursor = &cursor[8..];

            eprintln!("{} {} {}", typ, size, cursor.len());
            if typ > 11 {
                break;
            }

            match typ {
                4 => {
                    // Transform block
                    let data = &cursor[..size - 8];
                    let (name, skip) = read_str(data).unwrap();
                    let data = &data[skip..];
                    let idk = bytemuck::pod_read_unaligned(&data[..4]);
                    // let coords = bytemuck::pod_read_unaligned(&data[4..40]);
                    let pos = bytemuck::pod_read_unaligned(&data[4..4 + 12]);
                    let rot = bytemuck::pod_read_unaligned(&data[4 + 12..4 + 24]);
                    let scale = bytemuck::pod_read_unaligned(&data[4 + 24..40]);
                    ret.push(Block::Transform(TransformBlock {
                        name,
                        idk,
                        coords: TRS3d { pos, rot, scale },
                    }))
                }
                11 => {
                    // Bone
                    let data = &cursor[..size - 8];
                    let (name, skip) = read_str(data).unwrap();
                    let data = &data[skip..];
                    let idk = bytemuck::pod_read_unaligned(&data[..4]);
                    let coords = bytemuck::pod_read_unaligned(&data[4..40]);
                    let data = &data[40..];
                    let mut c = 0;
                    let mut matrix = [[0f32; 4]; 4];
                    for i in 0..4 {
                        for j in 0..4 {
                            if i == 3 {
                                if j == 3 {
                                    matrix[j][i] = 1f32;
                                } else {
                                    matrix[j][i] = 0f32;
                                }
                            } else {
                                matrix[j][i] = bytemuck::pod_read_unaligned(&data[c..c + 4]);
                                c += 4;
                            }
                        }
                    }
                    ret.push(Block::Bone(BoneBlock {
                        name,
                        idk,
                        coords,
                        matrix,
                    }))
                }
                0 => {
                    // Geometry?
                    let data = &cursor[..size - 8];
                    let (name, skip) = read_str(data).unwrap();
                    let data = &data[skip..];
                    let coords = bytemuck::pod_read_unaligned(&data[..16]);
                    let bool4 = bytemuck::pod_read_unaligned::<u32>(&data[16..20]) != 0;
                    let vertex_bitmask = bytemuck::pod_read_unaligned::<u32>(&data[20..24]);
                    let bool6 = bytemuck::pod_read_unaligned::<u32>(&data[24..28]) != 0;
                    let data = &data[28..];

                    let (idk, vertex_num, vertex_size, vertex_data) = if !bool6 {
                        // todo!();
                        let vertex_num = bytemuck::pod_read_unaligned::<u32>(&data[..4]);
                        const SIZES: [usize; 19] = [
                            12, 12, 12, 12, 16, 16, 4, 4, 4, 4, 16, 8, 8, 8, 8, 8, 8, 8, 8,
                        ];
                        let mut vertex_size = 0;
                        for i in 0..19 {
                            if (vertex_bitmask >> i) & 1 != 0 {
                                vertex_size += SIZES[i];
                            }
                        }
                        let vertex_data = &data[4..vertex_num as usize * vertex_size as usize];
                        let data = &data[4 + vertex_num as usize * vertex_size as usize..];

                        let size = bytemuck::pod_read_unaligned::<u32>(&data[..4]);
                        let mut data = &data[4..];
                        let mut ret = Vec::with_capacity(size as usize);

                        for _ in 0..size {
                            let render_type = bytemuck::pod_read_unaligned::<u32>(&data[..4]);
                            let w = bytemuck::pod_read_unaligned::<u32>(&data[4..8]);
                            let words = (0..w as usize)
                                .map(|x| &data[8 + 2 * x..8 + 2 * x + 2])
                                .map(|x| bytemuck::pod_read_unaligned(x))
                                .collect();
                            data = &data[8 + 2 * w as usize..];
                            ret.push(GeometryBlockInner {
                                typ: PTEnum::from(render_type),
                                words,
                            });
                        }
                        (ret, Some(vertex_num), Some(vertex_size), Some(vertex_data))
                    } else {
                        (Vec::new(), None, None, None)
                    };

                    // let mut idk = Vec::new();
                    ret.push(Block::Geometry(GeometryBlock {
                        name,
                        coords,
                        bool4,
                        vertex_bitmask: VertexMask {
                            mask: vertex_bitmask,
                        },
                        bool6,
                        idk,
                        vertex_num,
                        vertex_size,
                        vertex_data,
                    }))
                }
                _ => {
                    // Unknown block!
                    // TODO: replace
                    eprintln!("Unknown block {}", typ)
                }
            }

            cursor = &cursor[size - 8..];
        }

        ret
    }
}
