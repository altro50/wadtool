use std::env;
use std::fs::{metadata, File, create_dir_all, read_to_string};
use std::io::{Read, Write, Seek, SeekFrom};
use serde::{Deserialize, Serialize};
use crc::{crc32};

mod wads;

#[derive(Serialize, Deserialize)]
struct UnpackedInfo {
    name: String,
    files: Vec<String>
}

/**
 * input is the 2048 byte header, output is 512 u32 values. (256 offsets, 256 lengths)
 */
fn parse_wad_table(input: &[u8]) -> [u32; 512] {
    let mut ret = [0u32; 512];

    for i in 0 .. 256{
        ret[i] = 
                ((input[i * 4] as u32) <<  0) +
                ((input[i * 4 + 1] as u32) <<  8) +
                ((input[i * 4 + 2] as u32) << 16) +
                ((input[i * 4 + 3] as u32) << 24)
    }

    return ret;
}

fn pack(in_dir: &str, file_name: &str) {
    if !metadata(in_dir).is_ok() || !metadata(in_dir).unwrap().is_dir() {
        println!("! Cannot find directory: {}", in_dir);
        return;
    }

    let json_filename = format!("{}/wad.json", in_dir);

    if !metadata(&json_filename).is_ok() || !metadata(&json_filename).unwrap().is_file() {
        println!("! No wad.json in directory: {}", in_dir);
        return;
    }

    let wad_json = read_to_string(json_filename).expect("! Failed to load wad.json :/");
    let data: UnpackedInfo = serde_json::from_str(&wad_json).expect("! Badly formatted wad.json");
    let mut pos = 0;
    let mut dpos = 2048;

    println!("? Packing WAD file: {}.", data.name);

    let mut wad = File::create(file_name).expect("! Failed to create WAD file!");

    for file in &data.files {
        let fname = format!("{}/{}", in_dir, file);
        let file_size = metadata(&fname).expect(&format!("! Failed to open {}, make sure it exists!", &fname)).len();

        println!("? Writing file: {} ({} bytes)", fname, file_size);

        if file_size > 0 {
            // Write the header
            wad.seek(SeekFrom::Start(pos)).unwrap();

            wad.write(&[
                // Offset
                (dpos & 0xFF) as u8,
                ((dpos >> 8) & 0xFF) as u8,
                ((dpos >> 16) & 0xFF) as u8,
                ((dpos >> 24) & 0xFF) as u8,
                // File size
                (file_size & 0xFF) as u8,
                ((file_size >> 8) & 0xFF) as u8,
                ((file_size >> 16) & 0xFF) as u8,
                ((file_size >> 24) & 0xFF) as u8,
            ]).unwrap();

            let mut f_data = vec![0u8; file_size as usize];

            File::open(&fname).expect(&format!("! Failed to open {}, make sure it exists!", &fname)).read(&mut f_data).unwrap();

            wad.seek(SeekFrom::Start(dpos)).unwrap();

            wad.write_all(&f_data).expect(&format!("! Failed to read {}!", &fname));

            dpos += file_size;
        }

        pos += 8;
    }

    println!("✓ Successfully created WAD file {}", file_name);
}

fn unpack(file_name: &str, out_dir: &str) {
    if !metadata(file_name).is_ok() {
        println!("! Cannot find file: {}", file_name);
        return;
    }

    create_dir_all(out_dir).unwrap();

    let mut wad = File::open(file_name).unwrap();
    let mut header = vec![0; 2048];

    wad.read(&mut header).unwrap();

    let header_checksum = crc32::checksum_ieee(&header);
    let wad_table = parse_wad_table(&header);
    let mut last_file = 0;

    for i in 0 .. 256{
        // if the offset isn't 0
        if wad_table[i * 2] != 0 {
            last_file = i;
        }
    }

    println!("? Loaded {0}, containing {1} files.", file_name, last_file + 2);
    println!("? Header checksum: 0x{0:X}", header_checksum);

    let wad_details = wads::get_wad_by_checksum(header_checksum);

    if !wad_details.documented {
        println!("! Unknown WAD header checksum: 0x{0:X}, can't name files :c", header_checksum);
        println!("! If you happen to be good at reverse engineering, please contribute!");
    } else {
        println!("? Detected WAD: {}", wad_details.name);
    }

    let mut written_files = Vec::new();

    for i in 0 .. last_file + 1 {
        let offset = wad_table[i * 2];
        let length = wad_table[i * 2 + 1];
        let mut fname = format!("{}.wad", i);

        if wad_details.documented && i < wad_details.filenames.len() && wad_details.filenames[i].len() > 0 {
            fname = String::from(wad_details.filenames[i]);
        }

        println!("? {0}: offset: {1}, length: {2} (0x{1:X}-0x{3:X}) ({4})", i, offset, length, offset + length, fname);

        let mut unpack_handle = File::create(format!("{}/{}", out_dir, fname)).unwrap();

        // Vector to store the read data into
        let mut data = vec![0; length as usize];
        wad.seek(SeekFrom::Start(offset.into())).unwrap(); // Move the cursor to the offset
        wad.read(&mut data).unwrap();
        unpack_handle.write(&data).unwrap();
        written_files.push(fname);
    }

    println!("? Writing repack data to wad.json..");

    let mut repack_json = File::create(format!("{}/wad.json", out_dir)).unwrap();

    let repack_data = UnpackedInfo {
        name: String::from(wad_details.name),
        files: written_files
    };

    repack_json.write(serde_json::to_string_pretty(&repack_data).unwrap().as_bytes()).unwrap();

    println!("✓ Successfully extracted WAD file {}", file_name);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("? Wadtool, made by altro50, https://github.com/altro50/wadtool");
        println!("? Usage: {} pack (input directory) (wad name)", args[0]);
        println!("? Usage: {} unpack (wad name) (output directory)", args[0]);
        return;
    }

    match args[1].to_lowercase().as_str() {
        "unpack" => unpack(&args[2], &args[3]),
        "pack" => pack(&args[2], &args[3]),
        _ => println!("! Unknown operation {}..", args[1])
    }
}
