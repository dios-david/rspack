#![feature(map_many_mut)]
#![feature(let_chains)]

pub(crate) mod cache_group;
pub(crate) mod common;
pub(crate) mod module_group;
pub(crate) mod plugin;

pub use crate::{
  cache_group::CacheGroup,
  common::{
    create_all_chunk_filter, create_async_chunk_filter, create_chunk_filter_from_str,
    create_chunk_name_getter_by_const_name, create_default_module_filter,
    create_initial_chunk_filter, create_module_filter, create_module_filter_from_regex,
    create_module_filter_from_rspack_regex, SplitChunkSizes,
  },
  plugin::{PluginOptions, SplitChunksPlugin},
};
