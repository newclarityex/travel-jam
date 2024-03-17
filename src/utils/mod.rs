use bevy::prelude::*;
use bevy_collider_gen::Edges;
use bevy_rapier2d::geometry::Collider;

pub fn convex_decomposition(coords: Vec<Vec2>) -> Collider {
    let indices: Vec<[u32; 2]> = (0..coords.len()).map(|i| [i as u32, i as u32]).collect();
    let collider = Collider::convex_decomposition(&coords, &indices);
    collider
}

pub fn image_convex_decomposition(image: &Image) -> Collider {
    let edges = Edges::from(image);
    let coords = edges.single_image_edge_translated();
    let indices: Vec<[u32; 2]> = (0..coords.len()).map(|i| [i as u32, i as u32]).collect();
    let collider = Collider::convex_decomposition(&coords, &indices);
    collider
}
