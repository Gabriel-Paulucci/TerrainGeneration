use std::f32::consts::PI;

use amethyst::{
    assets::AssetLoaderSystemData,
    core::Transform,
    prelude::{Builder, WorldExt},
    renderer::{
        light::{Light, PointLight},
        loaders::load_from_linear_rgba,
        palette::{LinSrgba, Srgb},
        rendy::{
            hal::Primitive,
            mesh::{Color, MeshBuilder, PosColor, PosNorm, Position},
        },
        shape::InternalShape,
        Camera, Material, MaterialDefaults, Mesh, Texture,
    },
    shred::World,
    window::ScreenDimensions,
    SimpleState,
};

pub struct InitialState;

impl SimpleState for InitialState {
    fn on_start(&mut self, data: amethyst::StateData<'_, amethyst::GameData<'_, '_>>) {
        let world = data.world;

        init_camera(world);
        init_terrain(world);
        init_light(world);
    }
}

fn init_camera(world: &mut World) {
    let mut trasform = Transform::default();

    let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
    trasform.set_translation_xyz(0., 0., -4.);
    trasform.prepend_rotation_y_axis(PI);

    world
        .create_entity()
        .with(Camera::standard_3d(dimensions.width(), dimensions.height()))
        .with(trasform)
        .build();
}

fn init_terrain(world: &mut World) {
    let trasform = Transform::default();

    let mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        let verts: Vec<PosColor> = vec![
            PosColor {
                position: Position([-100., 0., 0.]),
                color: Color([1., 0., 0., 1.]),
            },
            PosColor {
                position: Position([100., 0., 0.]),
                color: Color([0., 1., 0., 1.]),
            },
            PosColor {
                position: Position([0., 100., 0.]),
                color: Color([0., 0., 1., 1.]),
            },
        ];

        let mesh_builder = MeshBuilder::from(verts)
            .with_indices(vec![0_u32, 1, 2])
            .with_prim_type(Primitive::TriangleList);

        // loader.load_from_data(
        //     Shape::Sphere(64, 64)
        //         .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
        //         .into(),
        //     (),
        // )

        loader.load_from_data(mesh_builder.into(), ())
    });

    // let material = {
    //     let material_defaults = world.read_resource::<MaterialDefaults>().0.clone();

    //     let albedo = world.exec(|loader: AssetLoaderSystemData<'_, Texture>| {
    //         loader.load_from_data(
    //             load_from_linear_rgba(LinSrgba::new(1.0, 1.0, 1.0, 0.5)).into(),
    //             (),
    //         )
    //     });

    //     world.exec(|loader: AssetLoaderSystemData<'_, Material>| {
    //         loader.load_from_data(
    //             Material {
    //                 albedo,
    //                 ..material_defaults
    //             },
    //             (),
    //         )
    //     })
    // };

    world
        .create_entity()
        .with(trasform)
        // .with(material)
        .with(mesh)
        .build();
}

fn init_light(world: &mut World) {
    let mut trasform = Transform::default();
    trasform.set_translation_xyz(6., -6., -6.);

    let light: Light = PointLight {
        intensity: 0.5,
        color: Srgb::new(0.0, 0.3, 0.7),
        ..Default::default()
    }
    .into();

    world.create_entity().with(trasform).with(light).build();
}
