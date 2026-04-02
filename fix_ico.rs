use std::fs;
use std::io::{Cursor, Read, Write};

fn main() {
    let input = "packaging/windows/icon.ico";
    let output = "packaging/windows/icon_bmp.ico";

    let data = fs::read(input).expect("Failed to read icon.ico");
    let mut cursor = Cursor::new(&data);

    // Read header
    let mut reserved = [0u8; 2];
    cursor.read_exact(&mut reserved).unwrap();
    let mut image_type = [0u8; 2];
    cursor.read_exact(&mut image_type).unwrap();
    let mut image_count = [0u8; 2];
    cursor.read_exact(&mut image_count).unwrap();

    let count = u16::from_le_bytes(image_count) as usize;
    println!("ICO contains {} images", count);

    // Read directory entries
    struct DirEntry {
        width: u8,
        height: u8,
        color_count: u8,
        reserved: u8,
        planes: u16,
        bit_count: u16,
        bytes_in_res: u32,
        image_offset: u32,
        is_png: bool,
    }

    let mut entries: Vec<DirEntry> = Vec::new();
    for _ in 0..count {
        let mut buf = [0u8; 16];
        cursor.read_exact(&mut buf).unwrap();
        let offset = u32::from_le_bytes(buf[12..16].try_into().unwrap());
        let size = u32::from_le_bytes(buf[8..12].try_into().unwrap());

        // Check if PNG compressed (signature at offset)
        let mut png_check = [0u8; 8];
        let pos = cursor.position();
        cursor.set_position(offset as u64);
        cursor.read_exact(&mut png_check).unwrap();
        cursor.set_position(pos);

        let is_png = png_check[0] == 0x89
            && png_check[1] == 0x50
            && png_check[2] == 0x4E
            && png_check[3] == 0x47;

        entries.push(DirEntry {
            width: buf[0],
            height: buf[1],
            color_count: buf[2],
            reserved: buf[3],
            planes: u16::from_le_bytes(buf[4..6].try_into().unwrap()),
            bit_count: u16::from_le_bytes(buf[6..8].try_into().unwrap()),
            bytes_in_res: size,
            image_offset: offset,
            is_png,
        });

        println!(
            "  {}x{}: {} bytes, PNG={}",
            if buf[0] == 0 { 256 } else { buf[0] as u32 },
            if buf[1] == 0 { 256 } else { buf[1] as u32 },
            size,
            is_png
        );
    }

    // Filter out PNG entries
    let bmp_entries: Vec<&DirEntry> = entries.iter().filter(|e| !e.is_png).collect();

    if bmp_entries.is_empty() {
        println!("ERROR: No BMP-based images found! Cannot create valid ICO.");
        std::process::exit(1);
    }

    println!("Keeping {} BMP-based images", bmp_entries.len());

    // Write new ICO
    let mut out = Vec::new();

    // Header
    out.write_all(&reserved).unwrap();
    out.write_all(&image_type).unwrap();
    out.write_all(&(bmp_entries.len() as u16).to_le_bytes())
        .unwrap();

    // Calculate data offset (header + directory)
    let data_offset = 6 + (bmp_entries.len() * 16) as u32;
    let mut current_offset = data_offset;

    // Write directory entries
    for entry in &bmp_entries {
        out.write_all(&[entry.width]).unwrap();
        out.write_all(&[entry.height]).unwrap();
        out.write_all(&[entry.color_count]).unwrap();
        out.write_all(&[entry.reserved]).unwrap();
        out.write_all(&entry.planes.to_le_bytes()).unwrap();
        out.write_all(&entry.bit_count.to_le_bytes()).unwrap();
        out.write_all(&current_offset.to_le_bytes()).unwrap();

        current_offset += entry.bytes_in_res;
    }

    // Write image data
    for entry in &bmp_entries {
        cursor.set_position(entry.image_offset as u64);
        let mut img_data = vec![0u8; entry.bytes_in_res as usize];
        cursor.read_exact(&mut img_data).unwrap();
        out.write_all(&img_data).unwrap();
    }

    fs::write(output, &out).expect("Failed to write output");
    println!("Wrote {} to {}", out.len(), output);
}
