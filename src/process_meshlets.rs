use std::{env, num::NonZero, path::Path};

use bevy::
{
    asset::
    {
        io::AssetSourceId,
        saver::
        {
            AssetSaver,
            SavedAsset,
        },
        AssetPath,
        ErasedLoadedAsset,
        LoadedAsset
    },
    pbr::experimental::meshlet::
    {
        MeshletMesh,
        MeshletMeshSaverLoad,
    },
    prelude::*,
    tasks::futures_lite::future,
};


pub struct ProcessMeshletsPlugin;

#[derive(Resource)]
struct AssetHandle(Handle<Mesh>);

impl Plugin for ProcessMeshletsPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(Startup, load_mesh);
        app.add_systems(Update, preprocess_meshlets);
    }
}

fn load_mesh(
    asset_server: Res<AssetServer>,
    mut handle_resource: ResMut<AssetHandle>,
)
{
    let args: Vec<String> = env::args().collect();
    let path: &Path = Path::new(&args[1]);
    let asset_path: &AssetPath = &AssetPath::from_path(path);
    handle_resource.0 = asset_server.load(asset_path);
}

fn preprocess_meshlets(
    assets: Res<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    handle: Res<AssetHandle>,
    mut app_exit: ResMut<Events<AppExit>>,
)
{
    let args: Vec<String> = env::args().collect();
    let path: &Path = Path::new(&args[1]);
    if !path.exists()
    {
        eprintln!("Error preprocessing mesh into meshlet: no valid mesh asset at specified path.");
        app_exit.send(AppExit::Error(NonZero::new(1).unwrap()));
    }
    
    let mut file_name: String = "".to_string();
    file_name.push_str(&path.file_prefix().unwrap().to_str().unwrap());
    let target_path = &Path::new("generated/meshlet/").join(file_name.to_owned() + "_generated.meshlet_mesh");

    
    if let Some(target_mesh) = assets.get(&handle.0)
    {
        if let Ok(meshlet) = MeshletMesh::from_mesh(&target_mesh)
        {

            future::block_on(async
            {
                let saver = MeshletMeshSaverLoad;
                let source = asset_server.get_source(AssetSourceId::Default).unwrap();
                let writer = source.writer().unwrap();
                let mut write = writer
                    .write(&target_path)
                    .await
                    .unwrap();
        
                let loaded = LoadedAsset::from(meshlet);
                let erased = ErasedLoadedAsset::from(loaded);
                if let Some(saved) = SavedAsset::from_loaded(&erased)
                {
                    if let Err(error) = saver.save(&mut *write, saved, &()).await
                    {
                        eprintln!("Error serializing mesh into meshlet: {}", error);
                        app_exit.send(AppExit::Error(NonZero::new(1).unwrap()));
                    }
                }
            });
            println!("Finished preprocessing mesh into meshlet.");
            app_exit.send(AppExit::Success);
        }
    }
}