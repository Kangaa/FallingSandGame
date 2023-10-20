use std::borrow::Cow;
use bevy::asset::AssetServer;
use bevy::prelude::{World, FromWorld, Resource};
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderDevice;

#[derive(Resource)]
pub struct CellPipeline{
    init_pipeline: CachedComputePipelineId,
    update_pipeline: CachedComputePipelineId,
    texture_bind_group_layout: BindGroupLayout,
}

impl FromWorld for CellPipeline{
    fn from_world(world: &mut World) -> Self{
        let texture_bind_group_layout = world.resource::<RenderDevice>()
            .create_bind_group_layout(&BindGroupLayoutDescriptor{
                label: Some("Cellular automata bind group layout"),
                entries: &[BindGroupLayoutEntry{
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::StorageTexture {
                        access: StorageTextureAccess::ReadWrite,
                        format: TextureFormat::Rgba8Unorm,
                        view_dimension: TextureViewDimension::D2,
                    },
                    count: None,
                }],
            });
        let pipeline_cache = world.resource::<PipelineCache>();
        let shader = world.resource::<AssetServer>().load("shaders/falling_sand.wgsl");
        
        let init_pipeline = pipeline_cache.queue_compute_pipeline(
            ComputePipelineDescriptor{
                label: Some(Cow::from("Falling Sand init pipeline")),
                layout: vec![texture_bind_group_layout.clone()],
                push_constant_ranges: vec![],
                shader: shader.clone(),
                shader_defs: vec![],
                entry_point: Cow::from("init"),
            }
        );
        let update_pipeline = pipeline_cache.queue_compute_pipeline(
            ComputePipelineDescriptor{
                label: Some(Cow::from("Falling sand Update pipeline")),
                layout: vec![texture_bind_group_layout.clone()],
                push_constant_ranges: vec![],
                shader,
                shader_defs: vec![],
                entry_point: Cow::from("update"),
            }
        );
        CellPipeline{
            texture_bind_group_layout,
            init_pipeline,
            update_pipeline,
        }
    }
}