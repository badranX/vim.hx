use std::sync::atomic::{AtomicBool, Ordering};

use crate::commands::*;
use helix_core::graphemes::prev_grapheme_boundary;
use helix_core::line_ending::rope_is_line_ending;
use helix_core::{textobject, Range, RopeSlice, Selection, Transaction};

//use crate::commands::{collapse_selection, extend_to_line_bounds, select_mode, Context};
use helix_view::document::Mode;

#[derive(Default)]
pub struct AtomicState {
    visual_lines: AtomicBool,
}

pub static VIM_STATE: AtomicState = AtomicState::new();

impl AtomicState {
    pub const fn new() -> Self {
        Self {
            visual_lines: AtomicBool::new(false),
        }
    }

    pub fn visual_line(&self) {
        self.visual_lines.store(true, Ordering::Relaxed);
    }

    pub fn exit_visual_line(&self) {
        self.visual_lines.store(false, Ordering::Relaxed);
    }

    pub fn is_visual_line(&self) -> bool {
        self.visual_lines.load(Ordering::Relaxed)
    }
}

pub struct VimOps;

impl VimOps {
    pub fn hook_after_each_command(cx: &mut Context) {
        if cx.editor.mode != Mode::Select {
            collapse_selection(cx);
        } else {
            // check if visual lines
            if VIM_STATE.is_visual_line() {
                extend_to_line_bounds(cx);
            }
        }
    }
}

macro_rules! wrap_hooks {
    // with both before and after
    ($wrapper:ident, $func:path, before = $before:expr, after = $after:expr) => {
        pub fn $wrapper(cx: &mut Context) {
            $before(cx);
            $func(cx);
            $after(cx);
        }
    };

    // with only before
    ($wrapper:ident, $func:path, before = $before:expr) => {
        pub fn $wrapper(cx: &mut Context) {
            $before(cx);
            $func(cx);
        }
    };

    // with only after
    ($wrapper:ident, $func:path, after = $after:expr) => {
        pub fn $wrapper(cx: &mut Context) {
            $func(cx);
            $after(cx);
        }
    };
}

macro_rules! wrap_many_with_hooks {
    (
        [ $( ( $wrapper:ident, $func:path ) ),+ $(,)? ],
        before = $before:expr,
        after = $after:expr
    ) => {
        $(
            wrap_hooks!($wrapper, $func, before = $before, after = $after);
        )+
    };

    (
        [ $( ( $wrapper:ident, $func:path ) ),+ $(,)? ],
        before = $before:expr
    ) => {
        $(
            wrap_hooks!($wrapper, $func, before = $before);
        )+
    };

    (
        [ $( ( $wrapper:ident, $func:path ) ),+ $(,)? ],
        after = $after:expr
    ) => {
        $(
            wrap_hooks!($wrapper, $func, after = $after);
        )+
    };
}

#[macro_export]
macro_rules! static_commands_with_default {
    ($macro_to_call:ident! ( $($name:ident, $doc:literal,)* )) => {
        $macro_to_call! {
        vim_visual_lines, "Visual lines (vim)",
        vim_normal_mode, "Normal mode (vim)",
        vim_exit_select_mode, "Exit select mode (vim)",
        vim_move_next_word_start, "Move to start of next word (vim)",
        vim_move_next_long_word_start, "Move next long word (vim)",
        vim_extend_next_word_start, "Extend to start of next word (vim)",
        vim_extend_next_long_word_start, "Extend to start of next long word (vim)",
        vim_extend_visual_line_up, "Move up (vim)",
        vim_extend_visual_line_down, "Move down (vim)",
        vim_goto_line, "Go to line (vim)",
        vim_move_paragraph_forward, "Move by paragraph forward (vim)",
        vim_move_paragraph_backward, "Move by paragraph forward (vim)",
        vim_delete, "Delete operator (vim)",
        vim_change, "Change operator (vim)",
        vim_yank, "Change operator (vim)",
        vim_yank_to_clipboard, "Change operator (vim)",
            $($name, $doc,)*
        }
    };
}

pub use vim_commands::*;

mod vim_commands {
    use vim_patch::exit_select_mode;

    use super::*;

    pub fn vim_visual_lines(cx: &mut Context) {
        select_mode(cx);
        VIM_STATE.visual_line();
        extend_to_line_bounds(cx);
    }

    wrap_many_with_hooks!(
        [
            (vim_move_next_word_start, move_next_word_start),
            (vim_move_next_long_word_start, move_next_long_word_start),
        ],
        after = move_char_right
    );

    wrap_many_with_hooks!(
        [
            (vim_extend_next_word_start, extend_next_word_start),
            (vim_extend_next_long_word_start, extend_next_long_word_start),
        ],
        after = extend_char_right
    );

    pub fn vim_goto_line(cx: &mut Context) {
        if cx.count.is_none() {
            goto_last_line(cx);
        } else {
            goto_line(cx);
        }
    }

    pub fn vim_extend_visual_line_down(cx: &mut Context) {
        if VIM_STATE.is_visual_line() {
            extend_line_down(cx);
        } else {
            extend_visual_line_down(cx);
        }
    }

    pub fn vim_extend_visual_line_up(cx: &mut Context) {
        if VIM_STATE.is_visual_line() {
            extend_line_up(cx);
        } else {
            extend_visual_line_up(cx);
        }
    }

    pub fn vim_normal_mode(cx: &mut Context) {
        normal_mode(cx);
        VIM_STATE.exit_visual_line();
    }

    pub fn vim_exit_select_mode(cx: &mut Context) {
        exit_select_mode(cx);
        VIM_STATE.exit_visual_line();
    }

    pub fn vim_move_paragraph_forward(cx: &mut Context) {
        goto_para_impl(cx, vim_utils::movement_paragraph_forward);
        if cx.editor.mode != Mode::Select {
            normal_mode(cx);
        }
    }

    pub fn vim_move_paragraph_backward(cx: &mut Context) {
        goto_para_impl(cx, vim_utils::movement_paragraph_backward);
        if cx.editor.mode != Mode::Select {
            normal_mode(cx);
        }
    }

    pub fn vim_delete(cx: &mut Context) {
        EvilOps::operator_impl(cx, EvilOperator::Delete, cx.register);
    }

    pub fn vim_yank(cx: &mut Context) {
        EvilOps::operator_impl(cx, EvilOperator::Yank, cx.register);
    }

    pub fn vim_yank_to_clipboard(cx: &mut Context) {
        EvilOps::operator_impl(cx, EvilOperator::Yank, Some('+'));
    }

    pub fn vim_change(cx: &mut Context) {
        EvilOps::operator_impl(cx, EvilOperator::Change, cx.register);
    }
}

mod vim_utils {
    use super::*;

    pub fn movement_paragraph_backward(
        slice: RopeSlice,
        range: Range,
        count: usize,
        behavior: Movement,
    ) -> Range {
        //Mostly copy/past from Movements::move_prev_paragraph
        let mut line = range.cursor_line(slice);
        let first_char = slice.line_to_char(line) == range.cursor(slice);
        let prev_line_empty = rope_is_line_ending(slice.line(line.saturating_sub(1)));
        let curr_line_empty = rope_is_line_ending(slice.line(line));
        let prev_empty_to_line = prev_line_empty && !curr_line_empty;

        // skip character before paragraph boundary
        if prev_empty_to_line && !first_char {
            line += 1;
        }
        let mut lines = slice.lines_at(line);
        lines.reverse();
        let mut lines = lines.map(rope_is_line_ending).peekable();
        let mut last_line = line;
        for _ in 0..count {
            while lines.next_if(|&e| e).is_some() {
                line -= 1;
            }
            while lines.next_if(|&e| !e).is_some() {
                line -= 1;
            }
            if lines.next_if(|&e| e).is_some() {
                line -= 1;
            }
            if line == last_line {
                break;
            }
            last_line = line;
        }

        let head = slice.line_to_char(line);
        let anchor = if behavior == Movement::Move {
            // exclude first character after paragraph boundary
            if prev_empty_to_line && first_char {
                range.cursor(slice)
            } else {
                range.head
            }
        } else {
            range.put_cursor(slice, head, true).anchor
        };
        Range::new(anchor, head)
    }

    pub fn movement_paragraph_forward(
        slice: RopeSlice,
        range: Range,
        count: usize,
        behavior: Movement,
    ) -> Range {
        //Mostly copy/paste from Movements::move_next_paragraph
        let mut line = range.cursor_line(slice);
        let last_char =
            prev_grapheme_boundary(slice, slice.line_to_char(line + 1)) == range.cursor(slice);
        let curr_line_empty = rope_is_line_ending(slice.line(line));
        let next_line_empty =
            rope_is_line_ending(slice.line(slice.len_lines().saturating_sub(1).min(line + 1)));
        let curr_empty_to_line = curr_line_empty && !next_line_empty;

        // skip character after paragraph boundary
        if curr_empty_to_line && last_char {
            line += 1;
        }
        let mut lines = slice.lines_at(line).map(rope_is_line_ending).peekable();
        let mut last_line = line;
        for _ in 0..count {
            while lines.next_if(|&e| e).is_some() {
                line += 1;
            }
            while lines.next_if(|&e| !e).is_some() {
                line += 1;
            }
            if lines.next_if(|&e| e).is_some() {
                line += 1;
            }
            if line == last_line {
                break;
            }
            last_line = line;
        }
        let head = slice.line_to_char(line);
        let anchor = if behavior == Movement::Move {
            if curr_empty_to_line && last_char {
                range.head
            } else {
                range.cursor(slice)
            }
        } else {
            range.put_cursor(slice, head, true).anchor
        };
        Range::new(anchor, head)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EvilOperator {
    Yank,
    Delete,
    Change,
}

pub struct EvilOps;

impl EvilOps {
    fn get_full_line_selection(
        cx: &mut Context,
        count: usize,
        include_last_newline: bool,
    ) -> Selection {
        let (view, doc) = current!(cx.editor);

        return doc.selection(view.id).clone().transform(|range| {
            let text = doc.text().slice(..);

            let (start_line, end_line) = range.line_range(text);
            let start = text.line_to_char(start_line);

            let end = if include_last_newline {
                text.line_to_char((end_line + count).min(text.len_lines()))
            } else {
                line_end_char_index(&text, end_line + count - 1)
            };

            Range::new(start, end).with_direction(range.direction())
        });
    }

    fn yank_selection(editor: &mut Editor, selection: &Selection, register: Option<char>) {
        // adabted from commands::yank_impl
        let register = register.unwrap_or(editor.config().default_yank_register);

        let (_, doc) = current!(editor);
        let text = doc.text().slice(..);

        let values: Vec<String> = selection.fragments(text).map(Cow::into_owned).collect();
        let selections = values.len();

        match editor.registers.write(register, values) {
            Ok(_) => editor.set_status(format!(
                "yanked {selections} selection{} to register {register}",
                if selections == 1 { "" } else { "s" }
            )),
            Err(err) => editor.set_error(err.to_string()),
        }
    }

    fn delete_selection_without_yank(
        cx: &mut Context,
        selection: &Selection,
        _set_status_message: bool,
    ) {
        let (view, doc) = current!(cx.editor);
        let transaction = Transaction::change_by_selection(doc.text(), selection, |range| {
            (range.from(), range.to(), None)
        });

        doc.apply(&transaction, view.id);
    }

    fn run_operator(
        cx: &mut Context,
        cmd: EvilOperator,
        register: Option<char>,
        selection_to_yank: &Selection,
        selection_to_delete: &Selection,
    ) {
        Self::yank_selection(cx.editor, selection_to_yank, register);

        match cmd {
            EvilOperator::Delete | EvilOperator::Change => {
                Self::delete_selection_without_yank(cx, selection_to_delete, true);
            }
            _ => return,
        }

        if cmd == EvilOperator::Change {
            insert_mode(cx);
        }
    }

    fn run_operator_for_current_selection(
        cx: &mut Context,
        cmd: EvilOperator,
        register: Option<char>,
    ) {
        let (view, doc) = current!(cx.editor);
        let selection = doc.selection(view.id).clone();

        Self::run_operator(cx, cmd, register, &selection, &selection);
    }

    fn run_operator_lines(
        cx: &mut Context,
        cmd: EvilOperator,
        register: Option<char>,
        count: usize,
    ) {
        let selection = Self::get_full_line_selection(cx, count, true);
        if cmd != EvilOperator::Change {
            Self::run_operator(cx, cmd, register, &selection, &selection);
        } else {
            let delete_selection = Self::get_full_line_selection(cx, count, false);
            Self::run_operator(cx, cmd, register, &selection, &delete_selection);
        }
    }

    pub fn operator_impl(cx: &mut Context, cmd: EvilOperator, register: Option<char>) {
        if cx.editor.mode == Mode::Select {
            EvilOps::run_operator_for_current_selection(cx, cmd, register);
            exit_select_mode(cx);
            return;
        }

        let count = cx.count();

        cx.on_next_key(move |cx, event| {
            cx.editor.autoinfo = None;
            if let Some(ch) = event.char() {
                match ch {
                    'i' => vim_operate_textobject(cx, textobject::TextObject::Inside, cmd),
                    'a' => vim_operate_textobject(cx, textobject::TextObject::Around, cmd),
                    'd' => EvilOps::run_operator_lines(cx, cmd, register, count),
                    'y' => EvilOps::run_operator_lines(cx, cmd, register, count),
                    'c' => EvilOps::run_operator_lines(cx, cmd, register, count),
                    _ => (),
                }
            }
        })
    }
}

fn vim_operate_textobject(cx: &mut Context, objtype: textobject::TextObject, op: EvilOperator) {
    // adapted from select_textobject

    let count = cx.count();

    cx.on_next_key(move |cx, event| {
        cx.editor.autoinfo = None;
        if let Some(ch) = event.char() {
            let (view, doc) = current!(cx.editor);

            let loader = cx.editor.syn_loader.load();
            let text = doc.text().slice(..);

            let textobject_treesitter = |obj_name: &str, range: Range| -> Range {
                let Some(syntax) = doc.syntax() else {
                    return range;
                };
                textobject::textobject_treesitter(
                    text, range, objtype, obj_name, syntax, &loader, count,
                )
            };

            let textobject_change = |range: Range| -> Range {
                let diff_handle = doc.diff_handle().unwrap();
                let diff = diff_handle.load();
                let line = range.cursor_line(text);
                let hunk_idx = if let Some(hunk_idx) = diff.hunk_at(line as u32, false) {
                    hunk_idx
                } else {
                    return range;
                };
                let hunk = diff.nth_hunk(hunk_idx).after;

                let start = text.line_to_char(hunk.start as usize);
                let end = text.line_to_char(hunk.end as usize);
                Range::new(start, end).with_direction(range.direction())
            };
            let mut is_valid = true;
            let selection = doc.selection(view.id).clone().transform(|range| {
                match ch {
                    'w' => textobject::textobject_word(text, range, objtype, count, false),
                    'W' => textobject::textobject_word(text, range, objtype, count, true),
                    't' => textobject_treesitter("class", range),
                    'f' => textobject_treesitter("function", range),
                    'a' => textobject_treesitter("parameter", range),
                    'c' => textobject_treesitter("comment", range),
                    'T' => textobject_treesitter("test", range),
                    'e' => textobject_treesitter("entry", range),
                    'p' => textobject::textobject_paragraph(text, range, objtype, count),
                    'm' => textobject::textobject_pair_surround_closest(
                        doc.syntax(),
                        text,
                        range,
                        objtype,
                        count,
                    ),
                    'g' => textobject_change(range),
                    // TODO: cancel new ranges if inconsistent surround matches across lines
                    ch if !ch.is_ascii_alphanumeric() => textobject::textobject_pair_surround(
                        doc.syntax(),
                        text,
                        range,
                        objtype,
                        ch,
                        count,
                    ),
                    _ => {
                        is_valid = false;
                        range
                    }
                }
            });
            if is_valid {
                EvilOps::run_operator(cx, op, cx.register, &selection, &selection);
            }
        }
    });

    let title = match objtype {
        textobject::TextObject::Inside => "Match inside",
        textobject::TextObject::Around => "Match around",
        _ => return,
    };
    let help_text = [
        ("w", "Word"),
        ("W", "WORD"),
        ("p", "Paragraph"),
        ("t", "Type definition (tree-sitter)"),
        ("f", "Function (tree-sitter)"),
        ("a", "Argument/parameter (tree-sitter)"),
        ("c", "Comment (tree-sitter)"),
        ("T", "Test (tree-sitter)"),
        ("e", "Data structure entry (tree-sitter)"),
        ("m", "Closest surrounding pair (tree-sitter)"),
        ("g", "Change"),
        (" ", "... or any character acting as a pair"),
    ];

    cx.editor.autoinfo = Some(Info::new(title, &help_text));
}
