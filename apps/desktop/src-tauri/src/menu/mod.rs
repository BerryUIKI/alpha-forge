/**
 * Native Menu Bar Module
 *
 * Implements the native menu bar for the desktop application using Tauri 2 Menu API.
 * Provides a Codex-style menu structure with File, Edit, View, and Help menus.
 *
 * Menu Structure:
 * - File: New Workspace, Open Workspace, Save, Export, Exit
 * - Edit: Undo, Redo, Cut, Copy, Paste
 * - View: Toggle Left Sidebar, Toggle Right Sidebar, Zoom In, Zoom Out, Reset Zoom
 * - Help: Documentation, Keyboard Shortcuts, Report Issue, About
 *
 * @module menu
 * @version GUI-M1-2
 */

use tauri::{
    menu::{Menu, MenuBuilder, MenuItem, MenuItemBuilder, Submenu, SubmenuBuilder},
    AppHandle, Manager, Runtime,
};

/// Menu identifiers for event handling
pub mod ids {
    // File menu items
    pub const FILE_MENU: &str = "file_menu";
    pub const NEW_WORKSPACE: &str = "new_workspace";
    pub const OPEN_WORKSPACE: &str = "open_workspace";
    pub const SAVE: &str = "save";
    pub const EXPORT: &str = "export";
    pub const EXIT: &str = "exit";

    // Edit menu items
    pub const EDIT_MENU: &str = "edit_menu";
    pub const UNDO: &str = "undo";
    pub const REDO: &str = "redo";
    pub const CUT: &str = "cut";
    pub const COPY: &str = "copy";
    pub const PASTE: &str = "paste";

    // View menu items
    pub const VIEW_MENU: &str = "view_menu";
    pub const TOGGLE_LEFT_SIDEBAR: &str = "toggle_left_sidebar";
    pub const TOGGLE_RIGHT_SIDEBAR: &str = "toggle_right_sidebar";
    pub const ZOOM_IN: &str = "zoom_in";
    pub const ZOOM_OUT: &str = "zoom_out";
    pub const RESET_ZOOM: &str = "reset_zoom";

    // Help menu items
    pub const HELP_MENU: &str = "help_menu";
    pub const DOCUMENTATION: &str = "documentation";
    pub const KEYBOARD_SHORTCUTS: &str = "keyboard_shortcuts";
    pub const REPORT_ISSUE: &str = "report_issue";
    pub const ABOUT: &str = "about";
}

/// Create the File menu
fn create_file_menu<R: Runtime>(app: &AppHandle<R>) -> Result<Submenu<R>, String> {
    SubmenuBuilder::new(app, "File")
        .id(ids::FILE_MENU)
        .item(
            &MenuItemBuilder::with_id(ids::NEW_WORKSPACE, "New Workspace")
                .accelerator("CmdOrCtrl+N")
                .build(app)
                .map_err(|e| e.to_string())?,
        )
        .item(
            &MenuItemBuilder::with_id(ids::OPEN_WORKSPACE, "Open Workspace")
                .accelerator("CmdOrCtrl+O")
                .build(app)
                .map_err(|e| e.to_string())?,
        )
        .separator()
        .item(
            &MenuItemBuilder::with_id(ids::SAVE, "Save")
                .accelerator("CmdOrCtrl+S")
                .build(app)
                .map_err(|e| e.to_string())?,
        )
        .item(
            &MenuItemBuilder::with_id(ids::EXPORT, "Export")
                .accelerator("CmdOrCtrl+E")
                .build(app)
                .map_err(|e| e.to_string())?,
        )
        .separator()
        .item(
            &MenuItemBuilder::with_id(ids::EXIT, "Exit")
                .accelerator("CmdOrCtrl+Q")
                .build(app)
                .map_err(|e| e.to_string())?,
        )
        .build()
        .map_err(|e| e.to_string())
}

/// Create the Edit menu
fn create_edit_menu<R: Runtime>(app: &AppHandle<R>) -> Result<Submenu<R>, String> {
    SubmenuBuilder::new(app, "Edit")
        .id(ids::EDIT_MENU)
        .item(
            &MenuItemBuilder::with_id(ids::UNDO, "Undo")
                .accelerator("CmdOrCtrl+Z")
                .build(app)
                .map_err(|e| e.to_string())?,
        )
        .item(
            &MenuItemBuilder::with_id(ids::REDO, "Redo")
                .accelerator("CmdOrCtrl+Shift+Z")
                .build(app)
                .map_err(|e| e.to_string())?,
        )
        .separator()
        .item(
            &MenuItemBuilder::with_id(ids::CUT, "Cut")
                .accelerator("CmdOrCtrl+X")
                .build(app)
                .map_err(|e| e.to_string())?,
        )
        .item(
            &MenuItemBuilder::with_id(ids::COPY, "Copy")
                .accelerator("CmdOrCtrl+C")
                .build(app)
                .map_err(|e| e.to_string())?,
        )
        .item(
            &MenuItemBuilder::with_id(ids::PASTE, "Paste")
                .accelerator("CmdOrCtrl+V")
                .build(app)
                .map_err(|e| e.to_string())?,
        )
        .build()
        .map_err(|e| e.to_string())
}

/// Create the View menu
fn create_view_menu<R: Runtime>(app: &AppHandle<R>) -> Result<Submenu<R>, String> {
    SubmenuBuilder::new(app, "View")
        .id(ids::VIEW_MENU)
        .item(
            &MenuItemBuilder::with_id(ids::TOGGLE_LEFT_SIDEBAR, "Toggle Left Sidebar")
                .accelerator("CmdOrCtrl+1")
                .build(app)
                .map_err(|e| e.to_string())?,
        )
        .item(
            &MenuItemBuilder::with_id(ids::TOGGLE_RIGHT_SIDEBAR, "Toggle Right Sidebar")
                .accelerator("CmdOrCtrl+2")
                .build(app)
                .map_err(|e| e.to_string())?,
        )
        .separator()
        .item(
            &MenuItemBuilder::with_id(ids::ZOOM_IN, "Zoom In")
                .accelerator("CmdOrCtrl+=")
                .build(app)
                .map_err(|e| e.to_string())?,
        )
        .item(
            &MenuItemBuilder::with_id(ids::ZOOM_OUT, "Zoom Out")
                .accelerator("CmdOrCtrl+-")
                .build(app)
                .map_err(|e| e.to_string())?,
        )
        .item(
            &MenuItemBuilder::with_id(ids::RESET_ZOOM, "Reset Zoom")
                .accelerator("CmdOrCtrl+0")
                .build(app)
                .map_err(|e| e.to_string())?,
        )
        .build()
        .map_err(|e| e.to_string())
}

/// Create the Help menu
fn create_help_menu<R: Runtime>(app: &AppHandle<R>) -> Result<Submenu<R>, String> {
    SubmenuBuilder::new(app, "Help")
        .id(ids::HELP_MENU)
        .item(
            &MenuItemBuilder::with_id(ids::DOCUMENTATION, "Documentation")
                .accelerator("F1")
                .build(app)
                .map_err(|e| e.to_string())?,
        )
        .item(
            &MenuItemBuilder::with_id(ids::KEYBOARD_SHORTCUTS, "Keyboard Shortcuts")
                .accelerator("CmdOrCtrl+/")
                .build(app)
                .map_err(|e| e.to_string())?,
        )
        .separator()
        .item(
            &MenuItemBuilder::with_id(ids::REPORT_ISSUE, "Report Issue")
                .build(app)
                .map_err(|e| e.to_string())?,
        )
        .separator()
        .item(
            &MenuItemBuilder::with_id(ids::ABOUT, "About Investment OS")
                .build(app)
                .map_err(|e| e.to_string())?,
        )
        .build()
        .map_err(|e| e.to_string())
}

/// Create and set up the main application menu
pub fn setup_menu<R: Runtime>(app: &AppHandle<R>) -> Result<Menu<R>, String> {
    // Create all submenus
    let file_menu = create_file_menu(app)?;
    let edit_menu = create_edit_menu(app)?;
    let view_menu = create_view_menu(app)?;
    let help_menu = create_help_menu(app)?;

    // Build the main menu
    MenuBuilder::new(app)
        .item(&file_menu)
        .item(&edit_menu)
        .item(&view_menu)
        .item(&help_menu)
        .build()
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_ids() {
        assert_eq!(ids::FILE_MENU, "file_menu");
        assert_eq!(ids::NEW_WORKSPACE, "new_workspace");
        assert_eq!(ids::VIEW_MENU, "view_menu");
        assert_eq!(ids::TOGGLE_LEFT_SIDEBAR, "toggle_left_sidebar");
    }
}
