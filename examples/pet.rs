#![feature(generic_associated_types)]
#![feature(generic_const_exprs)]
#![feature(adt_const_params)]

/// Matrix type.
pub struct Matrix4x4([f32; 16]);

pub struct SpriteData {
    // ...
}

/// Untyped pipeline layouts.
mod untyped_layout {
    mod descriptor_set {
        magma::untyped_descriptor_set_layout! {
            pub struct UniformBuffer0Vertex {
                0u32 (magma::pipeline::shader::stages::Vertex) => UniformBuffer
            }
        }
        
        magma::untyped_descriptor_set_layout! {
            pub struct UniformBuffer0Fragment {
                0u32 (magma::pipeline::shader::stages::Fragment) => UniformBuffer
            }
        }
    }
    
    mod descriptor_sets {
        use super::descriptor_set;

        magma::untyped_descriptor_set_layouts! {
            pub struct UniformBuffer0VertexOn0UniformBuffer0FragmentOn1 {
                0u32 : descriptor_set::UniformBuffer0Vertex ,
                1u32 : descriptor_set::UniformBuffer0Fragment
            }
        }
    }

    magma::untyped_pipeline_layout! {
        pub struct Sprite {
            type PushConstants = () ;
            type DescriptorSets = descriptor_sets::UniformBuffer0VertexOn0UniformBuffer0FragmentOn1 ;
        }
    }
}

/// Typed pipeline layouts.
mod layout {
    mod descriptor_set {
        magma::descriptor_set_layout! {
            pub struct TransformationOn0 {
                0u32 (magma::pipeline::shader::stages::Vertex) => magma::descriptor::ty::UniformBuffer<crate::Matrix4x4>
            }
        }
        
        magma::descriptor_set_layout! {
            pub struct SpriteOn0 {
                0u32 (magma::pipeline::shader::stages::Fragment) => magma::descriptor::ty::UniformBuffer<crate::SpriteData>
            }
        }
    }

    mod descriptor_sets {
        use super::descriptor_set;

        magma::descriptor_set_layouts! {
            pub struct BindTransformationOn0To0SpriteOn0To1 {
                0u32 : descriptor_set::TransformationOn0 ,
                1u32 : descriptor_set::SpriteOn0
            }
        }
    }

    magma::pipeline_layout! {
        pub struct Sprite : crate::untyped_layout::Sprite {
            type PushConstants = () ;
            type DescriptorSets = descriptor_sets::BindTransformationOn0To0SpriteOn0To1 ;
        }
    }
}

mod vertex_input {
    mod bindings {
        magma::vertex_input_bindings! {
            pub struct Sprite {
                0 => [f32; 2]
            }
        }
    }

    magma::vertex_input! {
        pub struct SpriteInput for bindings::Sprite {
            0 => 0
        }
    }
}

mod shader {
    mod descriptor_set {
        magma::shader_module_descriptor_set_layout! {
            pub struct VertexShaderTransformationSet {
                1 => magma::descriptor::ty::UniformBuffer<crate::Matrix4x4>
            }
        }
    }

    mod descriptor_sets {
        use super::descriptor_set;

        magma::shader_module_descriptor_set_layouts! {
            pub struct SpriteVertexShaderSets {
                0 => descriptor_set::VertexShaderTransformationSet
            }
        }
    }

    magma::shader_module_layout! {
        pub struct SpriteVertex {
            type PushConstants = ();
            type DescriptorSets = descriptor_sets::SpriteVertexShaderSets;
        }
    }

    mod stages {
        magma::pipeline_shader_stages! {
            pub struct SpriteShader : SpriteLayout {
                SpriteVertexShaderLayout
            }
        }
    }
}

mod pipeline {
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
}

// pub struct StaticViewportAndScissor;
// impl magma::pipeline::dynamic_state::ViewportsScissors for StaticViewportAndScissor {
//     type Viewports = magma::pipeline::dynamic_state::viewports::Static<1u32>;
//     type Scissors = magma::pipeline::dynamic_state::scissors::Static<1u32>;
// }
// pub struct StaticLineWidthAndDepthBias;
// impl magma::pipeline::dynamic_state::Rasterization for StaticLineWidthAndDepthBias {
//     type LineWidth = magma::pipeline::dynamic_state::line_width::Static;
//     type DepthBias = magma::pipeline::dynamic_state::depth_bias::Static;
// }
// pub struct StaticStencilTest;
// impl magma::pipeline::dynamic_state::StencilTest for StaticStencilTest {
//     type StencilCompareMask = magma::pipeline::dynamic_state::stencil_compare_mask::Static;
//     type StencilWriteMask = magma::pipeline::dynamic_state::stencil_write_mask::Static;
//     type StencilReference = magma::pipeline::dynamic_state::stencil_reference::Static;
// }

fn main() {
    println!("Hello World!")
}