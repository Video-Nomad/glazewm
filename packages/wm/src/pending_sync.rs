use std::collections::HashMap;

use uuid::Uuid;

use crate::{
  models::{Container, WindowContainer, Workspace},
  traits::CommonGetters,
};

#[derive(Debug, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct PendingSync {
  /// Containers (and their descendants) that have a pending redraw.
  containers_to_redraw: HashMap<Uuid, Container>,

  /// Workspaces where z-order should be updated. Windows that match the
  /// focused window's state should be brought to the front.
  workspaces_to_reorder: Vec<Workspace>,

  /// Whether native focus should be reassigned to the WM's focused
  /// container.
  needs_focus_update: bool,

  /// Windows where effects should be updated.
  windows_to_update_effects: HashMap<Uuid, WindowContainer>,

  /// Whether border colors should be updated after focus changes.
  needs_border_focus_update: bool,

  /// Whether window effects for all windows should be updated.
  needs_all_effects_update: bool,

  /// Whether to jump the cursor to the focused container (if enabled in
  /// user config).
  needs_cursor_jump: bool,
}

impl PendingSync {
  pub fn has_changes(&self) -> bool {
    !self.containers_to_redraw.is_empty()
      || !self.workspaces_to_reorder.is_empty()
      || self.needs_focus_update
      || !self.windows_to_update_effects.is_empty()
      || self.needs_border_focus_update
      || self.needs_all_effects_update
      || self.needs_cursor_jump
  }

  pub fn clear(&mut self) -> &mut Self {
    self.containers_to_redraw.clear();
    self.workspaces_to_reorder.clear();
    self.needs_focus_update = false;
    self.windows_to_update_effects.clear();
    self.needs_border_focus_update = false;
    self.needs_all_effects_update = false;
    self.needs_cursor_jump = false;
    self
  }

  pub fn queue_container_to_redraw<T>(&mut self, container: T) -> &mut Self
  where
    T: Into<Container>,
  {
    let container: Container = container.into();
    self.containers_to_redraw.insert(container.id(), container);
    self
  }

  pub fn queue_containers_to_redraw<I, T>(
    &mut self,
    containers: I,
  ) -> &mut Self
  where
    I: IntoIterator<Item = T>,
    T: Into<Container>,
  {
    for container in containers {
      let container: Container = container.into();
      self.containers_to_redraw.insert(container.id(), container);
    }

    self
  }

  pub fn dequeue_container_from_redraw<T>(
    &mut self,
    container: T,
  ) -> &mut Self
  where
    T: Into<Container>,
  {
    self.containers_to_redraw.remove(&container.into().id());
    self
  }

  pub fn queue_workspace_to_reorder(
    &mut self,
    workspace: Workspace,
  ) -> &mut Self {
    self.workspaces_to_reorder.push(workspace);
    self
  }

  pub fn queue_focus_change(&mut self) -> &mut Self {
    self.needs_focus_update = true;
    self
  }

  /// Queues a window for effect updates.
  pub fn queue_window_effect_update(
    &mut self,
    window: WindowContainer,
  ) -> &mut Self {
    self.windows_to_update_effects.insert(window.id(), window);
    self
  }

  /// Queues focused and previously focused windows for border updates.
  pub fn queue_border_focus_update(&mut self) -> &mut Self {
    self.needs_border_focus_update = true;
    self
  }

  pub fn queue_all_effects_update(&mut self) -> &mut Self {
    self.needs_all_effects_update = true;
    self
  }

  pub fn queue_cursor_jump(&mut self) -> &mut Self {
    self.needs_cursor_jump = true;
    self
  }

  pub fn needs_focus_update(&self) -> bool {
    self.needs_focus_update
  }

  pub fn needs_all_effects_update(&self) -> bool {
    self.needs_all_effects_update
  }

  /// Returns whether border colors should be updated after focus changes.
  pub fn needs_border_focus_update(&self) -> bool {
    self.needs_border_focus_update
  }

  pub fn needs_cursor_jump(&self) -> bool {
    self.needs_cursor_jump
  }

  pub fn containers_to_redraw(&self) -> &HashMap<Uuid, Container> {
    &self.containers_to_redraw
  }

  /// Returns windows where effects should be updated.
  pub fn windows_to_update_effects(
    &self,
  ) -> &HashMap<Uuid, WindowContainer> {
    &self.windows_to_update_effects
  }

  pub fn workspaces_to_reorder(&self) -> &Vec<Workspace> {
    &self.workspaces_to_reorder
  }
}
