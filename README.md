<div align="center">

# Dynamic Island

### Anime-Themed Dynamic Island for Desktop

**The sleekest desktop widget you've never had -- inspired by Apple's Dynamic Island, powered by Rust, themed in anime aesthetics.**

[![License: MIT](https://img.shields.io/badge/License-MIT-ff2d95.svg)](LICENSE)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-00f0ff.svg)](https://tauri.app)
[![Svelte](https://img.shields.io/badge/Svelte-5-FF3E00.svg)](https://svelte.dev)
[![Rust](https://img.shields.io/badge/Rust-🦀-DEA584.svg)](https://rust-lang.org)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20Windows-blueviolet.svg)]()
[![Stars](https://img.shields.io/github/stars/thanvish21/dynamic-island?style=social)](https://github.com/thanvish21/dynamic-island)

<br />

> A floating, always-on-top pill widget that brings iOS Dynamic Island to your Linux/Windows desktop — with anime sparkles, neon glows, and glassmorphism.

<br />

<!-- Screenshots -->
<!-- Add your screenshots here (see screenshots/README.md for the capture list) -->
<!-- ![Compact Pill](screenshots/compact.png) -->
<!-- ![Media Widget](screenshots/media.png) -->
<!-- ![System Monitor](screenshots/system.png) -->

</div>

---

## Features

### Media Control
- Now Playing — song title, artist, album art with spinning animation
- Play/pause, skip, previous controls
- Progress bar with neon gradient
- Supports Spotify, VLC, and any MPRIS-compatible player (Linux)

### System Monitor
- Real-time CPU & RAM usage with anime-styled progress rings
- Battery level with cute charging animation
- Network speed (upload/download)
- Disk usage overview

### Weather
- Current temperature & conditions
- Feels like, humidity, wind speed
- Animated weather effects (rain, snow, sun rays)
- Auto-location or manual coordinates

### Notifications
- System notification aggregation
- Slide-in notification cards
- Do Not Disturb mode with moon icon toggle
- Dismiss with poof animation

### Timer & Stopwatch
- Countdown timer with neon digits
- Pomodoro presets (25/5/15 min)
- Stopwatch with lap tracking
- Subtle pulse animation while running

### Clipboard History
- Last 50 clipboard entries
- Click to re-copy
- Clear history

### Anime Theme
- Deep purple + neon pink + cyan color palette
- Glassmorphism frosted glass effect
- Floating particle sparkles (sakura, stars)
- Neon glow borders that pulse
- Spring physics animations
- Kawaii-inspired icons and design

---

## Quick Start

### Prerequisites
- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) 1.70+
- [pnpm](https://pnpm.io/) 8+
- Linux: `webkit2gtk` and `librsvg` libraries
- Windows: WebView2 (usually pre-installed)

### Linux Dependencies

**Ubuntu/Debian**
```bash
sudo apt install libwebkit2gtk-4.1-dev librsvg2-dev patchelf
```

**Fedora**
```bash
sudo dnf install webkit2gtk4.1-devel librsvg2-devel
```

**Arch**
```bash
sudo pacman -S webkit2gtk-4.1 librsvg
```

### Install & Run

```bash
# Clone the repository
git clone https://github.com/thanvish21/dynamic-island.git
cd dynamic-island

# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev

# Build for production
pnpm tauri build
```

---

## Architecture

```
dynamic-island/
├── src/                    # Svelte 5 Frontend
│   ├── lib/
│   │   ├── components/     # UI Widgets
│   │   │   ├── Island.svelte          # Main container
│   │   │   ├── MediaWidget.svelte     # Now Playing
│   │   │   ├── SystemWidget.svelte    # CPU/RAM/Battery
│   │   │   ├── WeatherWidget.svelte   # Weather display
│   │   │   ├── NotificationWidget.svelte
│   │   │   ├── TimerWidget.svelte     # Timer/Stopwatch
│   │   │   ├── ClipboardWidget.svelte # Clipboard history
│   │   │   └── Particles.svelte       # Anime particles
│   │   ├── stores/         # Svelte stores
│   │   └── utils/          # Utilities
│   └── routes/             # SvelteKit pages
├── src-tauri/              # Rust Backend
│   └── src/
│       ├── lib.rs          # Tauri app builder
│       ├── system.rs       # System monitoring
│       ├── media.rs        # Media controls (MPRIS)
│       ├── weather.rs      # Weather API
│       ├── clipboard.rs    # Clipboard history
│       ├── notifications.rs # Notification center
│       └── timer.rs        # Timer/Stopwatch
└── .github/workflows/      # CI/CD
```

### Tech Stack

| Layer | Technology | Why |
|-------|-----------|-----|
| Runtime | **Tauri 2.0** | ~15MB RAM, native performance |
| Frontend | **Svelte 5** | Smallest bundle, best reactivity |
| Backend | **Rust** | Zero-cost abstractions, system APIs |
| Styling | **CSS** | Glassmorphism, animations, no deps |
| Weather | **Open-Meteo** | Free, no API key needed |
| Media | **MPRIS/playerctl** | Universal Linux media control |

---

## Controls

| Action | Result |
|--------|--------|
| Hover | Expand island |
| Click widget icon | Switch active widget |
| `Enter` (island focused) | Toggle expand/collapse |
| Move mouse away | Auto-collapse after 2 seconds |

---

## Customization

The theme colors live directly in the component styles under `src/lib/components/`. To reskin the island, search and replace the palette:

| Color | Hex |
|-------|-----|
| Neon pink | `#ff2d95` |
| Neon cyan | `#00f0ff` |
| Sakura | `#ffb7c5` |

---

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) first.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## License

This project is licensed under the MIT License -- see the [LICENSE](LICENSE) file for details.

---

## Acknowledgements

- [Tauri](https://tauri.app/) -- For making lightweight desktop apps possible
- [Svelte](https://svelte.dev/) -- For the best frontend DX
- [Open-Meteo](https://open-meteo.com/) -- Free weather API
- Apple -- For the Dynamic Island concept inspiration

---

<div align="center">

**Built with love by [Thanvish](https://github.com/thanvish21)**

*If you like this project, give it a star!*

</div>
