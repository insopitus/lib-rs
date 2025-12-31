use std::fs;

use crate::{
    geometry::TriMesh,
    linear_algebra::{vector::Vector2, Vector3},
};

pub struct Parser;
pub enum ParserError {
    Io(std::io::Error),
    InvalidVertexValue,
    InvalidFaceValue,
    InvalidNormalValue,
    InvalidTextureValue,
}

impl Parser {
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<TriMesh, ParserError> {
        let str = fs::read_to_string(path.as_ref()).map_err(ParserError::Io)?;
        let mut lines = str.lines();
        let mut positions: Vec<Vector3> = vec![];
        let mut normals: Vec<Vector3> = vec![];
        let mut uvs: Vec<Vector2> = vec![];
        // for line in lines {
        //     if line.starts_with("v ") {
        //         let mut value_iter = line.split_whitespace().skip(1);
        //         let x = value_iter
        //             .next()
        //             .map(|s| s.parse().map_err(|_| ParserError::InvalidVertexValue))
        //             .unwrap_or(Err(ParserError::InvalidVertexValue))?;
        //         let y = value_iter
        //             .next()
        //             .map(|s| s.parse().map_err(|_| ParserError::InvalidVertexValue))
        //             .unwrap_or(Err(ParserError::InvalidVertexValue))?;
        //         let z = value_iter
        //             .next()
        //             .map(|s| s.parse().map_err(|_| ParserError::InvalidVertexValue))
        //             .unwrap_or(Err(ParserError::InvalidVertexValue))?;

        //         positions.push(Vector3::new(x, y, z));
        //     } else if line.starts_with("vn ") {
        //         let mut value_iter = line.split_whitespace().skip(1);
        //         let x = value_iter
        //             .next()
        //             .map(|s| s.parse().map_err(|_| ParserError::InvalidNormalValue))
        //             .unwrap_or(Err(ParserError::InvalidNormalValue))?;
        //         let y = value_iter
        //             .next()
        //             .map(|s| s.parse().map_err(|_| ParserError::InvalidNormalValue))
        //             .unwrap_or(Err(ParserError::InvalidNormalValue))?;
        //         let z = value_iter
        //             .next()
        //             .map(|s| s.parse().map_err(|_| ParserError::InvalidNormalValue))
        //             .unwrap_or(Err(ParserError::InvalidNormalValue))?;

        //         normals.push(Vector3::new(x, y, z));
        //     } else if line.starts_with("vt ") {
        //         let mut value_iter = line.split_whitespace().skip(1);
        //         let x = value_iter
        //             .next()
        //             .map(|s| s.parse().map_err(|_| ParserError::InvalidTextureValue))
        //             .unwrap_or(Err(ParserError::InvalidTextureValue))?;
        //         let y = value_iter
        //             .next()
        //             .map(|s| s.parse().map_err(|_| ParserError::InvalidTextureValue))
        //             .unwrap_or(Err(ParserError::InvalidTextureValue))?;
        //         uvs.push(Vector2::new(x, y));
        //     } else if line.starts_with("f ") {
        //         let values: Vec<Vec<[usize; 3]>> = line
        //             .split_whitespace()
        //             .skip(1)
        //             .map(|s| {
        //                 let mut parts = s.split('/');
        //                 let v = parts
        //                     .next()
        //                     .unwrap()
        //                     .parse()
        //                     .map_err(|_| ParserError::InvalidFaceValue)?;
        //                 let vt = parts
        //                     .next()
        //                     .unwrap()
        //                     .parse()
        //                     .map_err(|_| ParserError::InvalidFaceValue)?;
        //                 let vn = parts
        //                     .next()
        //                     .unwrap()
        //                     .parse()
        //                     .map_err(|_| ParserError::InvalidFaceValue)?;
        //                 Ok([v, vt, vn])
        //             })
        //             .collect()?;
        //         todo!()
        //     }
        // }
        todo!()
    }
}
