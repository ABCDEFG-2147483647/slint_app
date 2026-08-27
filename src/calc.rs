pub fn calc_chunk_corner(coordinate: isize) -> isize {
    let coordinate_: f32 = coordinate as f32 / 16.0;
    coordinate_.floor() as isize * 16
}
