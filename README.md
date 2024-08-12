
# TK Notepad

TK Notepad is a simple, customizable text editor built with Rust using the `eframe` library. It features a file explorer for easy navigation and management of files and folders. 

## Features

- **Text Editing**: Create, open, and save text files with ease.
- **File Explorer**: Navigate your file system using a sidebar with icons and color-coded entries.
- **Dark/Light Mode**: Toggle between dark and light themes.
- **Show/Hide Hidden Files**: Optionally show or hide hidden files in the file explorer.
- **Undo/Redo**: Support for undo and redo actions.
- **Resizable Panels**: Resizable text area and scrollable file explorer.

## Installation

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd ThisNotepad
   ```

2. **Build the project**:
   Make sure you have Rust installed. You can install Rust using [rustup](https://rustup.rs/).

   ```bash
   cargo build
   ```

3. **Run the application**:
   ```bash
   cargo run
   ```

## Usage

### File Menu

- **New File**: Clears the current text editor for new input.
- **Open**: Opens a file dialog to select and load a text file into the editor.
- **Save**: Saves the current content to the opened file or prompts to save to a new file.
- **Save As**: Saves the current content to a new file.
- **Exit**: Closes the application.

### Edit Menu

- **Undo**: Reverts the last change made in the text editor.
- **Redo**: Reapplies a reverted change.

### View Menu

- **Show/Hide Sidebar**: Toggles the visibility of the file explorer sidebar.

### Settings Menu

- **Show Hidden Files**: Toggles the visibility of hidden files (files starting with a dot) in the file explorer.
- **Toggle Dark/Light Mode**: Switches between dark and light themes.

### Folder Menu

- **Open Folder**: Opens a folder dialog to select and display the contents in the file explorer.

### File Explorer

- **Navigate Folders**: Click on a folder to navigate into it.
- **Go to Parent Directory**: Click on `.. (Go to Parent Directory)` to navigate back to the parent directory.
- **Open Files**: Click on a file to open it in the text editor.

## Customization

The project is structured to allow easy modification of the UI and behavior. Feel free to explore and modify the `ui.rs` and `notepad_app.rs` files to customize the appearance and functionality.

## Contributing

Contributions are welcome! Please fork this repository, make your changes, and open a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
