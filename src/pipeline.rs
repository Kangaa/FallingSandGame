use std::borrow::Cow;
use bevy::app::{Plugin, App, Update, Startup};
use bevy::asset::AssetServer;
use bevy::prelude::{World, FromWorld, Resource, Image, Res, Commands, IntoSystemConfigs};
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::*;
use bevy::render::renderer::{RenderContext, RenderDevice};
use bevy::render::{Render, render_graph, RenderApp, RenderSet};
use bevy::render::render_graph::{NodeRunError, RenderGraph, RenderGraphContext};
use crate::cell_image::CellImage;
use crate::{SIMULATION_SIZE, WORKGROUP_SIZE};


pub struct PipelinesPlugin;
impl Plugin for PipelinesPlugin {
    fn build(&self, app: &mut App) {
        let render_app = match app.get_sub_app_mut(RenderApp) {
            Ok(render_app) => render_app,
            Err(_) => return,
        };

    }
    fn finish(&self, app: &mut App) {
        let render_app = match app.get_sub_app_mut(RenderApp) {
            Ok(render_app) => render_app,
            Err(_) => return,
        };

        render_app
            .add_systems(Render, queue_bind_group.in_set(RenderSet::Queue))
            .init_resource::<CellPipeline>();


        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        let node_id = render_graph.add_node("cell_graph", CellNode::default());
        render_graph.add_node_edge(node_id, bevy::render::main_graph::node::CAMERA_DRIVER);

    }
}



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

#[derive(Resource)]
struct CellImageBindGroup(pub BindGroup);

fn queue_bind_group(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    pipeline: Res<CellPipeline>,
    gpu_images: Res<RenderAssets<Image>>,
    cell_image: Res<CellImage>
){
    let view = &gpu_images[&cell_image.0];
    let bind_group = render_device.create_bind_group(&BindGroupDescriptor{
        label: Some("Cell Bind Group"),
        layout: &pipeline.texture_bind_group_layout,
        entries: &[BindGroupEntry{
            binding: 0,
            resource: BindingResource::TextureView(&view.texture_view),
        }],
    });
    commands.insert_resource(CellImageBindGroup(bind_group))
}

pub enum CellState{
    Loading,
    Init,
    Update,
}

pub struct CellNode{
    state: CellState
}

impl Default for CellNode{
    fn default() -> Self {
        Self {
            state: CellState::Loading,
        }
    }
}

impl render_graph::Node for CellNode{
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<CellPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        match self.state{
            CellState::Loading => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.init_pipeline){
                    self.state = CellState::Init
                }
            }
            CellState::Init => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.update_pipeline){
                    self.state = CellState::Update
                }
            }
            CellState::Update => {}
        }
    }

    fn run(&self,
           graph: &mut RenderGraphContext,
           render_context: &mut RenderContext,
           world: &World
    ) -> Result<(), NodeRunError> {
        let texture_bind_group = &world.resource::<CellImageBindGroup>().0;
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<CellPipeline>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());

        pass.set_bind_group(0, texture_bind_group, &[]);
        match self.state{
            CellState::Loading  => {}
            CellState::Init => {
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.init_pipeline)
                    .unwrap();
                pass.set_pipeline(init_pipeline);
                pass.dispatch_workgroups(
                    SIMULATION_SIZE.0 / WORKGROUP_SIZE,
                    SIMULATION_SIZE.1 / WORKGROUP_SIZE,
                    1
                )
            }
            CellState::Update => {
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.update_pipeline)
                    .unwrap();
                pass.set_pipeline(update_pipeline);
                pass.dispatch_workgroups(
                    SIMULATION_SIZE.0 / WORKGROUP_SIZE,
                    SIMULATION_SIZE.1 / WORKGROUP_SIZE,
                    1
                )
            }
        }
        Ok(())
    }
}
