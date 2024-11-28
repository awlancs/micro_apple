use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "../badapple.mp4".into();

    micro_apple_build::init().unwrap();
    let frames = micro_apple_build::generate_mini_frames(&path)?;

    let flattened = frames
        .into_iter()
        .flat_map(|f| f.0.map(|v| (v > 128) as u8))
        .collect::<Vec<_>>();
    println!("{:?}", &flattened[..100]);

    let mut encoded = Vec::new();

    let mut flat_iterator = flattened.into_iter();

    let mut last_value = flat_iterator.next().unwrap();
    let mut num_seen = 1;
    for value in flat_iterator {
        if value != last_value || num_seen == 0b0111_1111 {
            encoded.push((num_seen, last_value));
            last_value = value;
            num_seen = 0;
        }

        num_seen += 1;
    }
    encoded.push((num_seen, last_value));
    println!("{:?}", &encoded[..100]);
    
    let packed_encoded = encoded.into_iter().map(|(repeat, value)| repeat << 1 | value).collect::<Vec<_>>();
    println!("{:?}", &packed_encoded[..100]);


    let mut file = std::fs::File::create("../bad_apple_rle.bin")?;
    file.write_all(&packed_encoded)?;
    
    Ok(())
}