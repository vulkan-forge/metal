#![feature(generic_associated_types)]

magma::descriptor_set_layout! {
    pub struct TransformationOn0 {
        0u32 => UniformBuffer [1u32] (Vertex)
    }
}

magma::descriptor_set_layout! {
    pub struct SpriteOn0 {
        0u32 => UniformBuffer [1u32] (Fragment)
    }
}

magma::descriptor_set_layouts! {
    pub struct BindTransformationOn0To0SpriteOn0To1 {
        0u32 : TransformationOn0 ,
        1u32 : SpriteOn0
    }
}

magma::pipeline_layout! {
    pub struct SpriteLayout {
        type PushConstants = () ;
        type DescriptorSets = BindTransformationOn0To0SpriteOn0To1 ;
    }
}

magma::vertex_input_bindings! {
    pub struct SpriteInputBindings {
        0 => [f32; 2]
    }
}

magma::vertex_input! {
    pub struct SpriteInput for SpriteInputBindings {
        0 => 0
    }
}

pub struct Matrix4x4([f32; 16]);

magma::untyped_shader_module_descriptor_set! {
    pub struct Uniform0 {
        0 => [UniformBuffer; 1]
    }
}

magma::shader_module_descriptor_set! {
    pub struct VertexShaderTransformationSet : Uniform0 {
        1 => magma::descriptor::ty::UniformBuffer<Matrix4x4>
    }
}

// magma::graphics_pipeline! {
//     pub struct Sprite {
//         type Layout = std :: sync :: Arc < SpriteLayout > ;
//         type VertexInput = std :: sync :: Arc < SpriteInput > ;
//         type ViewportsScissors = StaticViewportAndScissor ;
//         type BlendConstants = magma :: pipeline :: dynamic_state :: blend_constants :: Dynamic ;
//         type Rasterization = StaticLineWidthAndDepthBias ;
//         type DepthBounds = magma :: pipeline :: dynamic_state :: depth_bounds :: Static ;
//         type StencilTest = StaticStencilTest ;
//         type RenderPass = std :: sync :: Arc < RenderPass > ;
//     }
// }

pub struct StaticViewportAndScissor;
impl magma::pipeline::dynamic_state::ViewportsScissors for StaticViewportAndScissor {
    type Viewports = magma::pipeline::dynamic_state::viewports::Static<1u32>;
    type Scissors = magma::pipeline::dynamic_state::scissors::Static<1u32>;
}
pub struct StaticLineWidthAndDepthBias;
impl magma::pipeline::dynamic_state::Rasterization for StaticLineWidthAndDepthBias {
    type LineWidth = magma::pipeline::dynamic_state::line_width::Static;
    type DepthBias = magma::pipeline::dynamic_state::depth_bias::Static;
}
pub struct StaticStencilTest;
impl magma::pipeline::dynamic_state::StencilTest for StaticStencilTest {
    type StencilCompareMask = magma::pipeline::dynamic_state::stencil_compare_mask::Static;
    type StencilWriteMask = magma::pipeline::dynamic_state::stencil_write_mask::Static;
    type StencilReference = magma::pipeline::dynamic_state::stencil_reference::Static;
}
fn main() {
    println!("Hello World!")
}