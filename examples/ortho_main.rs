use bevy::{prelude::*, render::camera::Camera};

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(bevy_tiled::TiledMapPlugin)
        .add_startup_system(setup.system())
        .add_system(animate_sprite_system.system())
        .add_system(camera_movement.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/gabe-idle-run.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 7, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let texture_handle = asset_server.load("assets/ortho.png").unwrap();
    commands
        .spawn(bevy_tiled::TiledMapComponents {
            map_asset: asset_server.load("assets/ortho-map.tmx").unwrap(),
            material: materials.add(texture_handle.into()),
            center: true,
            ..Default::default()
        })
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            scale: Scale(6.0),
            translation: Translation::new(425.0, 25.0, 1.0),
            draw: Draw {
                is_transparent: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1))
        .spawn(Camera2dComponents::default());
}

fn animate_sprite_system(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query.iter() {
        if timer.finished {
            let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
            timer.reset();
        }
    }
}

fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Camera, &mut Translation)>,
) {
    for (_, mut translation) in &mut query.iter() {
        let mut direction = Vec3::zero();
        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        translation.0 += time.delta_seconds * direction * 1000.0;
    }
}
