# Magma: A strictly typed Vulkan API interface.

Magma is a strictly typed Rust interface for the vulkan API.
This means that whenever possible,
the well formedness and safety of each vulkan object instance
is statically checked using the powerful type system of Rust,
using traits to define and verify required properties.
However because of the nature of the vulkan API,
it is nearly impossible to avoid unsafe trait implementations using Magma alone.
It is highly recommended to use Magma in combination with Cast,
a pipeline modelization framework that can be used to automatically
(and safely) implement unsafe traits.

Magma is highly inspired by vulkano, but closer to the original vulkan API
and including more static checks.
