# CrynN

**CrynN** is an ultra-lightweight, cross-platform browser built with a modern, Zen-like UI featuring vertical tabs by default. Built with Rust (Tauri) and TypeScript/React, it provides a lean, fast, and stable browsing experience powered by Firefox.

## Features

### MVP (Current)

- **Vertical Tabs**: Clean sidebar with favicon, title, audio indicator, close, pin, mute, drag-reorder, and new tab button
- **Tab Management**: Tab groups (collapse/expand), fuzzy tab search, context menus (duplicate, pin/unpin, mute/unmute, move to group, close others/right, reopen closed tab)
- **Navigation Controls**: Address bar with autocompletion (history/bookmarks), back/forward/reload/stop, home
- **Settings Page**: 
  - Themes: Light, Dark, Auto (OS), High-contrast with custom accent color
  - Keybindings: Fully remappable with conflict detection (new tab, close tab, next/prev tab, reopen closed, focus address bar, new window, tab search)
  - Privacy: Default search engine, Do-Not-Track toggle, telemetry OFF by default
  - Profiles: Create/select isolated Firefox profile for CrynN
  - Language: App UI locales (English shipped; infrastructure for more)
- **Bookmarks & History**: Bookmarks bar toggle, add/edit/delete, folders, import/export JSON; searchable history list
- **Downloads Panel**: Show progress, pause/resume, open in folder, clear list
- **Performance**: Cold start < 400ms (UI shell); shell memory < 120MB idle
- **Stability**: Reconnection UI if Firefox process closes/crashes; restore last session

### Phase 2 (Roadmap)

- Extensions surface (list installed Firefox add-ons for selected profile; deep link to AMO)
- Per-site permissions UI (read-only from profile data where feasible)
- Reader mode toggle; Picture-in-Picture quick control (if supported via command)

### Phase 3 (Roadmap)

- Workspaces; cloud sync (optional)
- Named session management
- Command palette (Ctrl/Cmd+K)

## Technology Stack

- **Desktop Shell**: Tauri
- **Backend**: Rust (commands, native ops, settings persistence)
- **Frontend**: TypeScript + React (Vite)
- **State Management**: Zustand
- **Styling**: Tailwind CSS with CSS variables for theming
- **IPC**: Tauri commands + events
- **Firefox Control**: WebDriver BiDi (preferred) with fallback to CDP via `geckodriver`/`webdriver-bidi` client
- **Persistence**: App config dir + JSON (optional sqlite via plugin)
- **Packaging**: Tauri bundler (Win/macOS/Linux)

## Prerequisites

### Required

- **Firefox** installed on your system
  - macOS: `/Applications/Firefox.app/Contents/MacOS/firefox`
  - Linux: Available via package manager or `which firefox`
  - Windows: `C:\Program Files\Mozilla Firefox\firefox.exe`

- **Node.js** (v18 or later) and **pnpm** (or npm/yarn)
- **Rust** (latest stable) and **Cargo**
- **System dependencies**:
  - macOS: Xcode Command Line Tools
  - Linux: `libwebkit2gtk-4.0-dev`, `libssl-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`
  - Windows: Microsoft Visual C++ Build Tools

### Optional (for BiDi/CDP fallback)

- `geckodriver` (for CDP fallback if BiDi fails)

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd Crynn
```

2. Install dependencies:
```bash
pnpm install
```

3. Build the project:
```bash
pnpm tauri build
```

Or run in development mode:
```bash
pnpm tauri dev
```

## Development

### Commands

- **Development**: `pnpm tauri dev` (starts Vite dev server + Tauri app)
- **Build**: `pnpm tauri build` (creates production builds for all platforms)
- **Test**: `pnpm test` (frontend tests) and `cargo test` (Rust tests)

### Project Structure

```
Crynn/
├── src-tauri/          # Rust backend
│   ├── src/
│   │   ├── main.rs     # Tauri setup, commands, event wiring
│   │   ├── bidi.rs     # Firefox BiDi client: launch, attach, commands
│   │   ├── config.rs   # Read/write JSON config, keybinding schema
│   │   └── downloads.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                # TypeScript/React frontend
│   ├── components/     # React components
│   │   ├── VerticalTabs.tsx
│   │   ├── AddressBar.tsx
│   │   ├── DownloadsPanel.tsx
│   │   └── Settings/
│   ├── stores/         # Zustand stores
│   │   ├── tabsStore.ts
│   │   ├── settingsStore.ts
│   │   ├── downloadsStore.ts
│   │   ├── bookmarksStore.ts
│   │   └── historyStore.ts
│   ├── lib/            # Utilities
│   │   ├── shortcuts.ts
│   │   ├── theme.ts
│   │   └── i18n.ts
│   ├── pages/          # Page components
│   ├── styles/         # CSS
│   ├── App.tsx
│   └── main.tsx
├── assets/             # Icons/logo
├── package.json
├── vite.config.ts
└── README.md           # This file (only markdown file)
```

## Default Keybindings

All keybindings are remappable in Settings:

- **New tab**: `Ctrl/Cmd+T`
- **Close tab**: `Ctrl/Cmd+W`
- **Next tab**: `Ctrl+Tab`
- **Previous tab**: `Ctrl+Shift+Tab`
- **Reopen closed**: `Ctrl/Cmd+Shift+T`
- **Focus address bar**: `Ctrl/Cmd+L`
- **New window**: `Ctrl/Cmd+N`
- **Tab search**: `Ctrl/Cmd+K`

## Firefox Control (BiDi/CDP)

CrynN controls Firefox via WebDriver BiDi (preferred) with a fallback to CDP (Chrome DevTools Protocol). The app:

1. Launches Firefox with an isolated profile directory managed by CrynN
2. Connects to Firefox's remote debugging port (default: 9222)
3. Uses BiDi/CDP to create/list/close/navigate tabs
4. Observes title/URL/audio events
5. Maintains mapping between UI Tab IDs and Firefox tab IDs

### Known Limitations

- **Tab Pinning**: True "pin" is not available via BiDi/CDP. CrynN emulates pinning in the UI and persists it locally.
- **Tab Muting**: Muting is emulated in the UI; Firefox's native mute may not be fully controllable via BiDi/CDP.
- **BiDi Feature Gaps**: Some features (e.g., pin/mute) are UI-level emulations. The actual Firefox tab state may differ.

### OS-Specific Notes

- **macOS**: Firefox is typically found at `/Applications/Firefox.app/Contents/MacOS/firefox`
- **Linux**: Firefox must be in PATH or specify full path in settings
- **Windows**: Firefox is typically at `C:\Program Files\Mozilla Firefox\firefox.exe`

If Firefox is not found, CrynN will display an error. You can configure a custom Firefox path in Settings.

## Performance Targets

- **Cold start**: < 400ms (UI shell)
- **Shell memory**: < 120MB idle
- **React bundle**: < 250KB gzipped (code-splitting enabled)

## UI/UX

- **Zen-like, low chrome**: 48px top bar, 280px default vertical tabs
- **Smooth animations**: <120ms; respects `prefers-reduced-motion`
- **Keyboard-first**: Address bar focus, tab cycling, command palette (Phase 3)
- **Icons**: Lucide React
- **Font**: System font stack
- **CSS variables**: `--bg`, `--fg`, `--muted`, `--accent`, `--border`

## License

MPL-2.0

## Contributing

Contributions are welcome! Please ensure:

- Code follows ESLint + Prettier rules
- Rust code passes `cargo clippy`
- Accessibility: keyboard nav, focus states, contrast AA+, reduced motion
- Tests pass: `pnpm test` and `cargo test`

## Troubleshooting

### Firefox not found

Ensure Firefox is installed and accessible. On macOS, check `/Applications/Firefox.app`. On Linux, ensure Firefox is in PATH or configure a custom path in Settings.

### BiDi/CDP connection fails

1. Check if Firefox is running: `is_firefox_running` command
2. Verify remote debugging port (default: 9222) is not blocked
3. Try restarting Firefox or CrynN
4. Check Firefox profile directory permissions

### Performance issues

- Ensure you're using the production build (`pnpm tauri build`)
- Check memory usage in Activity Monitor/Task Manager
- Disable extensions if using Firefox extensions

## Roadmap Summary

- ✅ **MVP**: Vertical tabs, navigation, settings, bookmarks, history, downloads, keybindings
- 🔄 **Phase 2**: Extensions surface, per-site permissions, reader mode, PiP
- 📋 **Phase 3**: Workspaces, cloud sync, named sessions, command palette

---

**CrynN** - Ultra-lightweight browser with a Zen-like UI.

