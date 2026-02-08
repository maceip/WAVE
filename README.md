Architectural Abstract: The Aurora Ribbon Component
The Aurora Ribbon is a high-density, task-oriented container that replaces traditional menus and toolbars. Architecturally, it is designed around the Command-Projection pattern, which separates the logic of an action from its visual representation.
1. Structural Abstractions
The Ribbon is organized into a strict hierarchy that enforces "Enterprise" discoverability:
•
RibbonTask (Tabs): The top-level organizational unit (e.g., "Page Layout", "Write"). Only one Task is active at a time, providing immediate context switching.
•
RibbonBand (Groups): Logical clusters of commands within a task (e.g., "Clipboard", "Font"). Bands handle their own layout and title labels.
•
Presentation Priority: Instead of fixed sizes, components are assigned priorities (Top, Medium, Low). The Ribbon uses these to decide whether to show a button as "Big" (icon + text), "Medium" (icon + text, small), or "Small" (icon only).
2. Advanced Component Patterns
The demo showcases specialized UI primitives that go beyond standard buttons:
•
In-Ribbon Galleries: Visual grids of options (like styles or colors) that scroll vertically within the horizontal ribbon. They can be expanded into full popups.
•
FlowRibbonBands: Dynamic groups that flow their content (like ComboBoxes and Checkboxes) into one, two, or three rows based on available vertical and horizontal space.
•
Contextual Task Groups: Tabs that only appear when specific conditions are met (e.g., a "Table Design" tab appearing only when a table is selected).
Description of the AuroraRibbonDemo
The AuroraRibbonDemo.kt is the definitive showcase of the library’s capabilities. It simulates a professional document editor to demonstrate complex state management.
Key Features in the Demo:
•
Dynamic Task Switching: Switches between "Page Layout", "Write", and "Animations" tasks. Each swap instantly replaces the entire toolbar content without reloading the window.
•
Contextual Logic: It demonstrates Contextual Task Groups (colored tabs like "Group 1" and "Group 2") that can be toggled on or off, showing how the UI adapts to different "Objects" being selected.
•
Adaptive Resizing: If you shrink the window, you can watch the "Discovery" band transition from full buttons to a single "Iconified" popup button. It uses Resize Policies to ensure the most important tools stay visible longest.
•
Application Menu: Implements the "File" button (Application Menu) which projects a two-pane layout for global commands like "Open", "Save", and "Print".
•
Keytip System: Pressing Alt triggers an overlay of letters across every button, allowing for full keyboard-driven navigation through the entire command hierarchy.
•
Rich Tooltips: Hovering over commands reveals multi-part tooltips containing titles, descriptions, and even diagrams, demonstrating the high-information density intended for enterprise users.
Technical Implementation:
The demo uses a RibbonBuilder class to declaratively define the entire command structure. It maintains a RibbonState (a Kotlin data class) that tracks which task is selected, which contextual groups are visible, and the current document styles, proving that even a massive UI can be driven by a single, clean state object.

Assets and Component Analysis of AuroraRibbonDemo
The AuroraRibbonDemo is a high-density "Enterprise" interface that uses a sophisticated grouping of specialized components. Below are the specific assets and component types categorized by their functional grouping in the Ribbon.
1. Visual Assets (Icons)
The demo uses a large set of transcoded SVG icons (converted into Compose Painter objects) from several libraries:
•
Material Design: Used for utility actions (e.g., refresh_black, history_black, person_black, help_outline).
•
Tango: Used for professional editor actions (e.g., edit_paste, format_indent_less, applications_office, address_book_new).
•
Vaadin: Used for specialized layout and application icons.
•
Custom Geometric Painters: Used in the "Style Gallery" to render dynamic previews of document styles (rectangles with text and lines).
2. Component Types & Interaction Models
The UI is built using Projections, where a data model is "projected" into a specific visual state.
•
CommandButton (Projections):
◦
Big: Icon on top, text on bottom (e.g., "Paste", "Push to GCR").
◦
Medium: Icon on left, text on right (e.g., "Cut", "Copy").
◦
Small: Icon only (visible when the window shrinks).
◦
Toggle Buttons: Used for formatting (Bold, Italic) and SimCity blocks (Active Messes).
•
Selector Components:
◦
CheckBoxProjection: Used in the "Show/Hide" band for "Ruler", "Gridlines", etc.
◦
RadioButtonProjection: (Implicitly used via SelectorContentModel) for mutual exclusivity in "Presentation" modes (Comfortable vs. Compact).
•
Input Components:
◦
ComboBoxProjection: Used in the "Font" band for selecting "Font Family" and "Font Size."
•
Complex Layout Components:
◦
CommandButtonStrip: Groups related actions horizontally (e.g., Alignment: Left, Center, Right, Fill).
◦
In-Ribbon Gallery: A scrollable grid of visual options (Style 1, Style 2, etc.) embedded directly in the Ribbon.
◦
CommandButtonPanel: Used for the "SimCity" grid, allowing for a 2D layout of project blocks.
3. Functional Grouping (Hierarchy)
Components are grouped into Bands, which are then grouped into Tasks.
•
The "Clipboard" Band:
◦
Contains CommandButtons (Paste, Cut, Copy) and a CommandButtonPanel inside a popup menu for "Format".
•
The "Font" Band (Flow Band):
◦
Uses a Flow layout. Contains two ComboBoxes and three CommandStrips. These automatically wrap into 1, 2, or 3 rows depending on window width.
•
The "Show/Hide" Band (Component Group):
◦
Contains a list of CheckBoxes (Ruler, Gridlines, Message Bar, etc.).
•
The "Presentation" Band:
◦
Contains a group of mutually exclusive Selectors (Comfortable, Cozy, Compact).
•
The "Universe" Task:
◦
Groups high-level discovery tools (Scan for Life, GitHub Pulse) and the main CommandPanel for managing projects.
4. Theming & Decoration Area Types
Assets are visually grouped using Decoration Areas to provide visual hierarchy:
•
TitlePane: Holds the "Taskbar" commands (Save, Print).
•
Header: The Ribbon task toggle area.
•Source Files for the Aurora Ribbon Demo
The Aurora Ribbon Demo is composed of several Kotlin source files and resource files within the :demo module. Below is the list of files categorized by their role in the application.
1. Core Application Logic
These files contain the main entry point, the UI structure, and the state management for the demo.
•
demo\src\desktopMain\kotlin\org\pushingpixels\aurora\demo\ribbon\AuroraRibbonDemo.kt: The primary file. It contains the main function, the RibbonBuilder class (which defines the entire ribbon structure), and the layout logic for the window content.
•
demo\src\desktopMain\kotlin\org\pushingpixels\aurora\demo\ribbon\RibbonState.kt: Defines the data structures and enums used to track the application's state (e.g., selected task, document styles, font settings).
2. Resource & Localization Files
These files provide the text labels, tooltips, and descriptions used throughout the UI.
•
demo\src\desktopMain\resources\org\pushingpixels\aurora\demo\Resources.properties: The default English localized strings.
•
demo\src\desktopMain\resources\org\pushingpixels\aurora\demo\Resources_iw.properties: The Hebrew localized strings (used to demonstrate RTL support).
3. Visual Assets (Transcoded SVGs)
The demo relies on hundreds of generated Kotlin files that represent SVG icons. These are located in:
•
demo\src\desktopMain\kotlin\org\pushingpixels\aurora\demo\svg\tango\: Professional editor icons (e.g., edit_paste.kt, format_indent_less.kt).
•
demo\src\desktopMain\kotlin\org\pushingpixels\aurora\demo\svg\material\: Utility icons (e.g., refresh_black_24dp.kt, person_black_24dp.kt).
•
demo\src\desktopMain\kotlin\org\pushingpixels\aurora\demo\svg\vaadin\: Application-specific icons.
•
demo\src\desktopMain\kotlin\org\pushingpixels\aurora\demo\svg\radiance_menu.kt: The main application menu icon.
4. Shared Demo Components
The demo occasionally uses shared primitives from the main demo package:
•
demo\src\desktopMain\kotlin\org\pushingpixels\aurora\demo\ColorIcons.kt: (If applicable) helper classes for rendering color-related icons.
5. Build Configuration
•
demo\build.gradle.kts: Configures the dependencies (Compose, Aurora libraries) and defines the mainClass for running the demo.
In summary, while the core "brain" of the demo lives in just two files (AuroraRibbonDemo.kt and RibbonState.kt), it orchestrates hundreds of asset files and localized resources to create the full enterprise experience.
ControlPane: The main body of the Ribbon bands.
•
None (Default): The main "Universe" canvas.
This structure allows the app to maintain 50+ distinct assets while keeping the interface discoverable and responsive.
