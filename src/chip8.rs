use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
    time::{Duration, Instant},
};

use winit::{
    event::{ElementState, Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::{
    display::{build_pixels, build_window},
    emulator::{execute, fetch_instruction, match_key, KeyState},
    memory::Memory,
    program_counter::ProgramCounter,
    registers::Registers,
    stack::Stack,
};

fn roms_directory() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("roms")
}

#[cfg(target_os = "macos")]
fn pick_rom_file(roms_dir: &Path) -> Option<PathBuf> {
    let default_clause = if roms_dir.is_dir() {
        format!(
            " default location (POSIX file \"{}\")",
            roms_dir.display()
        )
    } else {
        String::new()
    };
    let script = format!(
        "POSIX path of (choose file with prompt \"Select a CHIP-8 ROM\"{default_clause})"
    );
    let output = Command::new("osascript")
        .args(["-e", &script])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if path.is_empty() {
        return None;
    }
    Some(PathBuf::from(path))
}

#[cfg(all(unix, not(target_os = "macos")))]
fn pick_rom_file(roms_dir: &Path) -> Option<PathBuf> {
    let mut cmd = Command::new("zenity");
    cmd.args(["--file-selection", "--title=Select a CHIP-8 ROM"]);
    if roms_dir.is_dir() {
        cmd.arg(format!("--filename={}/", roms_dir.display()));
    }
    let output = cmd.output().ok()?;
    if !output.status.success() {
        return None;
    }
    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if path.is_empty() {
        return None;
    }
    Some(PathBuf::from(path))
}

#[cfg(windows)]
fn pick_rom_file(roms_dir: &Path) -> Option<PathBuf> {
    pick_rom_interactive(roms_dir)
}

fn pick_rom_interactive(roms_dir: &Path) -> Option<PathBuf> {
    let mut entries: Vec<PathBuf> = fs::read_dir(roms_dir)
        .ok()?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_file())
        .collect();
    entries.sort();
    if entries.is_empty() {
        eprintln!("No ROM files found in {}", roms_dir.display());
        return None;
    }
    for (i, p) in entries.iter().enumerate() {
        let label = p
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| p.display().to_string());
        eprintln!("  {}) {}", i + 1, label);
    }
    print!("Enter number (1-{}): ", entries.len());
    let _ = io::stdout().flush();
    let mut line = String::new();
    io::stdin().read_line(&mut line).ok()?;
    let n: usize = line.trim().parse().ok()?;
    entries.get(n.checked_sub(1)?).cloned()
}

#[cfg(not(any(
    target_os = "macos",
    all(unix, not(target_os = "macos")),
    windows
)))]
fn pick_rom_file(_roms_dir: &Path) -> Option<PathBuf> {
    None
}

/// Ensures `memory` contains ROM bytes and fonts; returns ROM length for the fetch step.
fn ensure_rom_loaded(
    rom_from_args: &mut Option<Vec<u8>>,
    memory: &mut Memory,
    roms_dir: &Path,
) -> Option<usize> {
    let rom_bytes = if let Some(buf) = rom_from_args.take() {
        buf
    } else {
        let picked = pick_rom_file(roms_dir).or_else(|| pick_rom_interactive(roms_dir))?;
        fs::read(&picked).unwrap_or_else(|e| {
            panic!("Could not read ROM at {}: {e}", picked.display());
        })
    };

    let rom_len = rom_bytes.len();
    memory.set_rom(&rom_bytes).unwrap();
    memory.set_fonts();
    *rom_from_args = Some(rom_bytes);
    Some(rom_len)
}

pub fn chip8(width: u32, height: u32, mut rom_from_args: Option<Vec<u8>>) {
    let event_loop = EventLoop::new();
    let scale = 20;
    let window = build_window(width * scale, height * scale, &event_loop);
    let mut pixels = build_pixels(&window, width, height).unwrap();

    const INSTRUCTIONS_PER_SECOND: u32 = 700;
    let time_per_instruction = Duration::from_secs(1) / INSTRUCTIONS_PER_SECOND;

    let mut stack = Stack::new();
    let mut program_counter = ProgramCounter::new();
    program_counter.set_counter(512);

    let mut registers = Registers::new();
    let mut memory = Memory::new();

    let roms_dir = roms_directory();
    let mut rom_ready = rom_from_args.is_some();
    if rom_ready {
        let r = rom_from_args.as_ref().unwrap();
        memory.set_rom(r).unwrap();
        memory.set_fonts();
    }

    // CHIP-8 keypad key (0x0..=0xF) currently held, if any.
    let mut held_chip8_key: Option<u8> = None;

    // main event loop
    event_loop.run(move |event, _, control_flow| {
        let start_time = Instant::now();
        *control_flow = ControlFlow::WaitUntil(start_time + time_per_instruction);

        match event {
            // Event::MainEventsCleared case signifies that all the events which were available up to the point of the last call to the event handler have been processed and the event loop is ready to proceed to the next phase of the loop's body.
            Event::MainEventsCleared => {
                if !rom_ready {
                    match ensure_rom_loaded(&mut rom_from_args, &mut memory, &roms_dir) {
                        Some(_) => rom_ready = true,
                        None => {
                            println!("No ROM selected; exiting.");
                            control_flow.set_exit();
                            return;
                        }
                    }
                }

                let rom_length = rom_from_args.as_ref().map(|r| r.len()).unwrap_or(0);
                let instruction =
                    fetch_instruction(memory.get_memory(), &mut program_counter, rom_length);

                let keys = KeyState {
                    held_key: held_chip8_key,
                };
                execute(
                    instruction,
                    &mut memory,
                    &mut stack,
                    &mut registers,
                    &mut program_counter,
                    &mut pixels,
                    width,
                    keys,
                );
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                control_flow.set_exit();
            }
            Event::RedrawRequested(_) => {
                pixels.render().unwrap();
            }
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(_virtual_keycode),
                            scancode: key_scancode,
                            ..
                        },
                    ..
                } => {
                    held_chip8_key = match_key(key_scancode);
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Released,
                            virtual_keycode: Some(_key),
                            ..
                        },
                    ..
                } => {
                    held_chip8_key = None;
                }
                _ => {}
            },
            _ => {}
        }
    });
}
