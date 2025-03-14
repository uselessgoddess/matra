pub mod textures;

pub fn dev() -> bool {
  cfg!(debug_assertions)
}

pub fn gizmos() -> bool {
  false
}
