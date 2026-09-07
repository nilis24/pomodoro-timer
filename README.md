# Pomodoro Timer

An open source desktop application for planning and running Pomodoro sessions based on the time you have available.

The project is written in Rust and uses [`eframe`](https://crates.io/crates/eframe) / `egui` for the graphical interface.

## Features

- Pomodoro planning based on available time.
- Automatic calculation of work sessions, short breaks, and long breaks.
- Option to add an extra session with the remaining time when it meets the configured minimum.
- Timer with pause, resume, reset, and stop controls.
- Configuration for work duration, short break, long break, long break frequency, and minimum extra session length.
- Initial screens for planning by number of cycles and viewing a time log.

## Project status

The project is in an early stage. Available-time planning and timer execution are already functional. Some screens, such as the time log and cycle-based planning, are currently prepared as a foundation for future improvements.

## Requirements

- Rust installed with `cargo`.

You can install Rust from [rustup.rs](https://rustup.rs/).

## Installation

Clone the repository and enter the project directory:

```bash
git clone <repository-url>
cd pomodoro-timer
```

Build the project:

```bash
cargo build
```

## Usage

Run the application with:

```bash
cargo run
```

From the main screen, you can enter your available time, review the generated plan, and start the timer. The default values are:

- 25 minutes of work.
- 5-minute short break.
- 15-minute long break.
- Long break every 4 cycles.
- Minimum extra session of 10 minutes.

These values can be adjusted from the settings tab.

## Development

To run the tests:

```bash
cargo test
```

Main structure:

- `src/main.rs`: application entry point.
- `src/app.rs`: main application state and screen navigation.
- `src/business/pomodoro.rs`: Pomodoro plan calculation and execution logic.
- `src/screens/`: interface screens.
- `src/ui_helpers.rs`: shared interface helpers.

## Contributing

Contributions are welcome. Before opening an issue or pull request, please review [`CONTRIBUTING.md`](CONTRIBUTING.md).

We also ask everyone participating in the project to follow [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).

## License

This project is distributed under the license specified in [`LICENSE`](LICENSE).
