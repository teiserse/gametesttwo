use amethyst::assets::{Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{Camera, Projection};
use amethyst::{
    core::{
        nalgebra::{Vector2, Vector3},
    },
    ecs::prelude::World,
    renderer::{
        Material, MaterialDefaults, MeshHandle, PosTex,
    },
};
const SIZE: f32 = 1500.0;

pub struct GameTwo;
impl SimpleState for GameTwo {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<Tri>();
        
        initialise_tri(world);
        initialise_camera(world);
    }
}

pub struct Tri {
    pub size: f32,
}

impl Tri {
    fn new(size: f32) -> Tri {
        Tri { size }
    }
}

impl Component for Tri {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0, 750.0, 0.0, 750.0,
        )))
        .with(transform)
        .build();
}

fn initialise_tri(world: &mut World) {
    let mut tri_transform = Transform::default();
    tri_transform.set_xyz(0.0, 0.0, 0.0);

    let tri_mesh = create_mesh(
        world,
        vec![
            PosTex {
                position: Vector3::new(SIZE, 0.0, 0.0),
                tex_coord: Vector2::new(0.0, 0.0),
            },
            PosTex {
                position: Vector3::new(-SIZE, 0.0, 0.0),
                tex_coord: Vector2::new(0.0, 0.0),
            },
            PosTex {
                position: Vector3::new(0.0, SIZE * 1.73205, 0.0),
                tex_coord: Vector2::new(1.0, 1.0),
            },
        ],
    );

    let tri_material = create_colour_material(world, [0.2, 0.7, 0.5, 1.0]);

    world
        .create_entity()
        .with(tri_mesh)
        .with(tri_material)
        .with(Tri::new(SIZE))
        .with(tri_transform)
        .build();
}

/// Creates a solid material of the specified colour.
fn create_colour_material(world: &World, colour: [f32; 4]) -> Material {
    // TODO: optimize
    let mat_defaults = world.read_resource::<MaterialDefaults>();
    let loader = world.read_resource::<Loader>();

    let albedo = loader.load_from_data(colour.into(), (), &world.read_resource());

    Material {
        albedo,
        ..mat_defaults.0.clone()
    }
}

/// Converts a vector of vertices into a mesh.
fn create_mesh(world: &World, vertices: Vec<PosTex>) -> MeshHandle {
    let loader = world.read_resource::<Loader>();
    loader.load_from_data(vertices.into(), (), &world.read_resource())
}
