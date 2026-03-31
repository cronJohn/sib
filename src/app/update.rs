use crate::context::Context;
use crate::message::Message;
use crate::model::Model;
use crate::panels::filter::FilterItem;

pub fn update(model: &mut Model, msg: Message, _ctx: &Context) {
    use Message::*;

    match msg {
        // Global events
        Quit => model.should_quit = true,
        CycleFocusForward => {
            model.panel_focus = model.panel_focus.next();
        }

        // Input panel events
        SwitchInputMode(mode) => {
            model.input_panel.mode = mode;
        }

        InputChar(c) => model.input_panel.buffer.push(c),
        InputBackspace => {
            model.input_panel.buffer.pop();
        }

        AddFilter => {
            let item_to_filter = model
                .input_panel
                .get_filter_item()
                .expect("Unable to get filter item");
            match item_to_filter {
                FilterItem::Slug(name) => model.filter_criteria.slug = Some(name),
                FilterItem::Tag(tag) => model.filter_criteria.tags.push(tag),
                FilterItem::Meta(k, v) => {
                    model.filter_criteria.metadata.insert(k, v);
                }
            }

            model.recompute_filtered();

            model.input_panel.buffer.clear();
        }

        // Tree panel events
        TreeSelectionUp => {
            model.tree_move_up();
        }

        TreeSelectionDown => {
            model.tree_move_down();
        }

        OpenSelected => {
            if let Some(_note) = model.get_selected_note() {
                todo!();
            }
        }

        // Filter panel events
        DeleteSelectedFilter => {
            let items = model.build_filter_items();

            if let Some(item) = items.get(model.filter_panel.selection_index.get()) {
                match item {
                    FilterItem::Slug(_) => model.filter_criteria.slug = None,
                    FilterItem::Tag(tag) => {
                        model.filter_criteria.tags.retain(|t| t != tag);
                    }
                    FilterItem::Meta(k, _) => {
                        model.filter_criteria.metadata.remove(k);
                    }
                }

                model.recompute_filtered();

                // clamp selection
                let len = model.build_filter_items().len();
                model.filter_panel.selection_index.down(len);
            }
        }

        FilterSelectionUp => {
            model.filter_panel.selection_index.up();
        }
        FilterSelectionDown => {
            let len = model.build_filter_items().len();
            model.filter_panel.selection_index.down(len);
        }

        Noop => {}
    }
}
