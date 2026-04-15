use std::{
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
};

use weaverail_model::{metadata::Metadata, model::DiagramRoot};

use crate::WeaverailIoError;

const MAGIC_NUMBER: &[u8; 4] = b"#DIA";

/// マジックナンバーが一致しているか
fn check_magic_number(magic: &[u8; 4]) -> bool {
    magic == MAGIC_NUMBER
}

/// ファイルがWeaverailObjectであるかどうか
pub fn is_file_weaverail(path: PathBuf) -> io::Result<bool> {
    let mut file = File::open(path)?;
    let mut magic = [0u8; 4];
    file.read_exact(&mut magic)?;
    Ok(check_magic_number(&magic))
}

/// Weaverailプロジェクトファイルを読み込む関数
pub fn read_file(path: &PathBuf) -> Result<(DiagramRoot, Metadata), WeaverailIoError> {
    let mut file = File::open(path)?;
    let mut magic = [0u8; 4];
    file.read_exact(&mut magic)?;
    if !check_magic_number(&magic) {
        return Err(WeaverailIoError::InvalidMagicNumber);
    }
    let mut metadata_size = [0u8; 4];
    file.read_exact(&mut metadata_size)?;
    let mut metadata_raw = vec![0u8; u32::from_be_bytes(metadata_size) as usize];
    file.read_exact(&mut metadata_raw)?;
    let metadata = ron::de::from_reader(metadata_raw.as_slice())?;

    let decompressed = zstd::decode_all(file)?;
    let root = ron::de::from_reader(decompressed.as_slice())?;

    Ok((root, metadata))
}

/// Weaverailプロジェクトファイルを書き込む関数
pub fn write_file(
    path: &PathBuf,
    root: &DiagramRoot,
    metadata: &Metadata,
) -> Result<(), WeaverailIoError> {
    let mut file = File::create(path)?;

    let metadata = ron::ser::to_string(&metadata)?;
    let metadata = metadata.as_bytes();
    let metadata_length = metadata.len() as u32;

    let root = ron::ser::to_string(&root)?;
    let root = zstd::encode_all(root.as_bytes(), 0)?;

    file.write_all(MAGIC_NUMBER)?;
    file.write_all(&u32::to_be_bytes(metadata_length))?;
    file.write_all(metadata)?;
    file.write_all(&root)?;

    Ok(())
}

#[test]
fn write_read() {
    let test_data = weaverail_model::test_data::diagram_root::get_test_data();
    let path = PathBuf::from("./test.wvr");
    let _ = write_file(&path, &test_data.root, &test_data.metadata);
    let data = read_file(&path);
    let data = data.unwrap();
    println!("{:?}", data);
}
