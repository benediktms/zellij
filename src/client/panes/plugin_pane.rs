use crate::{common::SenderWithContext, pty_bus::VteBytes, tab::Pane, wasm_vm::PluginInstruction};

use crate::panes::{PaneId, PositionAndSize};

use std::time::Instant;
use std::{sync::mpsc::channel, unimplemented};

pub struct PluginPane {
    pub pid: u32,
    pub should_render: bool,
    pub selectable: bool,
    pub invisible_borders: bool,
    pub position_and_size: PositionAndSize,
    pub position_and_size_override: Option<PositionAndSize>,
    pub send_plugin_instructions: SenderWithContext<PluginInstruction>,
    pub max_height: Option<usize>,
    pub max_width: Option<usize>,
    pub active_at: Instant,
}

impl PluginPane {
    pub fn new(
        pid: u32,
        position_and_size: PositionAndSize,
        send_plugin_instructions: SenderWithContext<PluginInstruction>,
    ) -> Self {
        Self {
            pid,
            should_render: true,
            selectable: true,
            invisible_borders: false,
            position_and_size,
            position_and_size_override: None,
            send_plugin_instructions,
            max_height: None,
            max_width: None,
            active_at: Instant::now(),
        }
    }
}

impl Pane for PluginPane {
    // FIXME: These position and size things should all be moved to default trait implementations,
    // with something like a get_pos_and_sz() method underpinning all of them. Alternatively and
    // preferably, just use an enum and not a trait object
    fn x(&self) -> usize {
        self.position_and_size_override
            .unwrap_or(self.position_and_size)
            .x
    }
    fn y(&self) -> usize {
        self.position_and_size_override
            .unwrap_or(self.position_and_size)
            .y
    }
    fn rows(&self) -> usize {
        self.position_and_size_override
            .unwrap_or(self.position_and_size)
            .rows
    }
    fn columns(&self) -> usize {
        self.position_and_size_override
            .unwrap_or(self.position_and_size)
            .columns
    }
    fn reset_size_and_position_override(&mut self) {
        self.position_and_size_override = None;
        self.should_render = true;
    }
    fn change_pos_and_size(&mut self, position_and_size: &PositionAndSize) {
        self.position_and_size = *position_and_size;
        self.should_render = true;
    }
    // FIXME: This is obviously a bit outdated and needs the x and y moved into `size`
    fn override_size_and_position(&mut self, x: usize, y: usize, size: &PositionAndSize) {
        let position_and_size_override = PositionAndSize {
            x,
            y,
            rows: size.rows,
            columns: size.columns,
            ..Default::default()
        };
        self.position_and_size_override = Some(position_and_size_override);
        self.should_render = true;
    }
    fn handle_pty_bytes(&mut self, _event: VteBytes) {
        unimplemented!()
    }
    fn cursor_coordinates(&self) -> Option<(usize, usize)> {
        None
    }
    fn adjust_input_to_terminal(&self, _input_bytes: Vec<u8>) -> Vec<u8> {
        unimplemented!() // FIXME: Shouldn't need this implmented?
    }

    fn position_and_size_override(&self) -> Option<PositionAndSize> {
        self.position_and_size_override
    }
    fn should_render(&self) -> bool {
        self.should_render
    }
    fn set_should_render(&mut self, should_render: bool) {
        self.should_render = should_render;
    }
    fn selectable(&self) -> bool {
        self.selectable
    }
    fn set_selectable(&mut self, selectable: bool) {
        self.selectable = selectable;
    }
    fn set_invisible_borders(&mut self, invisible_borders: bool) {
        self.invisible_borders = invisible_borders;
    }
    fn set_max_height(&mut self, max_height: usize) {
        self.max_height = Some(max_height);
    }
    fn set_max_width(&mut self, max_width: usize) {
        self.max_width = Some(max_width);
    }
    fn render(&mut self) -> Option<String> {
        // if self.should_render {
        if true {
            // while checking should_render rather than rendering each pane every time
            // is more performant, it causes some problems when the pane to the left should be
            // rendered and has wide characters (eg. Chinese characters or emoji)
            // as a (hopefully) temporary hack, we render all panes until we find a better solution
            let (buf_tx, buf_rx) = channel();

            self.send_plugin_instructions
                .send(PluginInstruction::Render(
                    buf_tx,
                    self.pid,
                    self.rows(),
                    self.columns(),
                ))
                .unwrap();

            self.should_render = false;
            Some(buf_rx.recv().unwrap())
        } else {
            None
        }
    }
    fn pid(&self) -> PaneId {
        PaneId::Plugin(self.pid)
    }
    fn reduce_height_down(&mut self, count: usize) {
        self.position_and_size.y += count;
        self.position_and_size.rows -= count;
        self.should_render = true;
    }
    fn increase_height_down(&mut self, count: usize) {
        self.position_and_size.rows += count;
        self.should_render = true;
    }
    fn increase_height_up(&mut self, count: usize) {
        self.position_and_size.y -= count;
        self.position_and_size.rows += count;
        self.should_render = true;
    }
    fn reduce_height_up(&mut self, count: usize) {
        self.position_and_size.rows -= count;
        self.should_render = true;
    }
    fn reduce_width_right(&mut self, count: usize) {
        self.position_and_size.x += count;
        self.position_and_size.columns -= count;
        self.should_render = true;
    }
    fn reduce_width_left(&mut self, count: usize) {
        self.position_and_size.columns -= count;
        self.should_render = true;
    }
    fn increase_width_left(&mut self, count: usize) {
        self.position_and_size.x -= count;
        self.position_and_size.columns += count;
        self.should_render = true;
    }
    fn increase_width_right(&mut self, count: usize) {
        self.position_and_size.columns += count;
        self.should_render = true;
    }
    fn push_down(&mut self, count: usize) {
        self.position_and_size.y += count;
    }
    fn push_right(&mut self, count: usize) {
        self.position_and_size.x += count;
    }
    fn pull_left(&mut self, count: usize) {
        self.position_and_size.x -= count;
    }
    fn pull_up(&mut self, count: usize) {
        self.position_and_size.y -= count;
    }
    fn scroll_up(&mut self, _count: usize) {
        unimplemented!()
    }
    fn scroll_down(&mut self, _count: usize) {
        unimplemented!()
    }
    fn clear_scroll(&mut self) {
        unimplemented!()
    }
    fn max_height(&self) -> Option<usize> {
        self.max_height
    }
    fn max_width(&self) -> Option<usize> {
        self.max_width
    }
    fn invisible_borders(&self) -> bool {
        self.invisible_borders
    }

    fn active_at(&self) -> Instant {
        self.active_at
    }

    fn set_active_at(&mut self, time: Instant) {
        self.active_at = time;
    }
}
