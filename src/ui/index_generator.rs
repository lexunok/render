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
pub fn generate_partial_ring() -> Vec<u16> {

    let mut indices: Vec<u16> = Vec::new();

    indices.extend_from_slice(&[0, 1, 2, 0, 2, 3]);
    let mut is_draw = true;

    for i in (2..362).step_by(2) {
        if i % 20 == 0 {
            is_draw = !is_draw;
        }
        if is_draw {
            indices.push(i + 1);
            indices.push(i);
            indices.push(i + 2);
            indices.push(i + 1);
            indices.push(i + 2);
            indices.push(i + 3);   
        }
    };

    indices
}
pub fn generate_circle() -> Vec<u16> {
    let mut indices = Vec::new();
    for i in 1..361 {
        indices.push(i);
        indices.push(0);
        indices.push(i + 1);
    };
    indices
}