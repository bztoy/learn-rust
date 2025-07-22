fn get_pallet_size() -> Vec<u8> {
    let s: Vec<u8> = vec![8, 16, 32];
    return s;
}

pub fn list_pallet_size() {
    let pallets = get_pallet_size();

    for p in pallets.iter() {
        println!("the size of pallet is {}", p);
    }
}
