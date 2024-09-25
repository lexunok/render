pub fn generate_ring() -> Vec<u16> {

    let mut indices: Vec<u16> = Vec::new();

    indices.extend_from_slice(&[0, 1, 2, 0, 2, 3]);

    for i in (2..362).step_by(2) {
        indices.push(i + 1);
        indices.push(i);
        indices.push(i + 2);
        indices.push(i + 1);
        indices.push(i + 2);
        indices.push(i + 3);
    };

    indices
}
