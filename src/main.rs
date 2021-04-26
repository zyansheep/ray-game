#![allow(unused_imports, unused_import_braces, dead_code)]
use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, input::mouse::MouseMotion, prelude::*, reflect::TypeUuid, render::{camera::Camera, pipeline::{PipelineDescriptor, RenderPipeline}, render_graph::{AssetRenderResourcesNode, RenderGraph, base}, renderer::RenderResources, shader::{ShaderStage, ShaderStages}}};

mod player;
use player::Player;
mod camera;
use camera::{CameraFocusEvent, CameraInterpolation, OrbitCameraPlugin};

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 8 })
        .add_plugins(DefaultPlugins)
        .add_asset::<MyMaterial>()
        // Orbit Camera Plugin
        .add_plugin(OrbitCameraPlugin::new(
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(-3.0, 5.0, 0.0),
        ))
        .add_startup_system(setup.system())
        .add_system(player_movement.system())
        .run();
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "1e08866c-0b8a-437e-8bce-37733b25127e"]
struct MyMaterial {
    pub color: Color,
}

const VERTEX_SHADER: &str = r#"
#version 450
layout(location = 0) in vec3 Vertex_Position;
layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
void main() {
    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
}
"#;

const FRAGMENT_SHADER: &str = r#"
#version 450
layout(location = 0) out vec4 o_Target;
layout(set = 2, binding = 0) uniform MyMaterial_color {
    vec4 color;
};
void main() {
    o_Target = color;
}
"#;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MyMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    // Create a new shader pipeline
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
    }));

    // Add an AssetRenderResourcesNode to our Render Graph. This will bind MyMaterial resources to
    // our shader
    render_graph.add_system_node(
        "my_material",
        AssetRenderResourcesNode::<MyMaterial>::new(true),
    );

    // Add a Render Graph edge connecting our new "my_material" node to the main pass node. This
    // ensures "my_material" runs before the main pass
    render_graph
        .add_node_edge("my_material", base::node::MAIN_PASS)
        .unwrap();

    // Create a new material
    let my_material = materials.add(MyMaterial {
        color: Color::rgb(0.0, 0.8, 0.0),
    });

    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        //material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    // character
    commands.spawn_bundle(Player::bundle(
        "You",
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        },
    )).insert(my_material);
    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn player_movement(
    mut focus_broadcast: EventWriter<CameraFocusEvent>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_transform: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = player_transform.single_mut().unwrap();

    let mut moved = false;
    if keyboard_input.pressed(KeyCode::W) {
        //info!("'W' currently pressed");
        moved = true;
        player_transform.translation.x += 0.1;
    }
    if keyboard_input.pressed(KeyCode::S) {
        //info!("'S' currently pressed");
        moved = true;
        player_transform.translation.x -= 0.1;
    }
    if keyboard_input.pressed(KeyCode::A) {
        //info!("'A' currently pressed");
        moved = true;
        player_transform.translation.z -= 0.1;
    }
    if keyboard_input.pressed(KeyCode::D) {
        //info!("'A' currently pressed");
        moved = true;
        player_transform.translation.z += 0.1;
    }
    if moved {
        focus_broadcast.send(CameraFocusEvent::new(
            player_transform.translation,
            CameraInterpolation::None,
        ));
    }
}

/* fn camera_movement_system(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut q: QuerySet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<Camera>>
    )>
) {
    let player_transform = q.q0().single().unwrap().clone();
    let mut camera_transform = q.q1_mut().single_mut().unwrap();

    camera_transform.look_at(player_transform.translation, Vec3::new(0.,1.,0.));
} */
