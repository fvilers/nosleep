# nosleep

A small software that prevents your Windows computer to enter sleep mode

## How does it work

`nosleep.exe` will periodically send a keyboard input that mimics pressing the `F15` key. The default duration between two inputs is 60 seconds.

## Install

I don't intend to publish this package on [crates.io](https://crates.io/), but you can install it directly from its repository.

```bash
cargo install --git https://github.com/fvilers/nosleep.git
```

## Usage

Manually execute the binary `nosleep.exe` (should be located in `%USERPROFILE%\.cargo\bin\nosleep.exe`) or ask Windows to execute it on startup.

### Automatic execution

You have several choices to execute `nosleep.exe` automatically:

#### Startup folder

Create a symbolic link from the executable in the Startup folder in `%APPDATA%\microsoft\Windows\Start Menu\Programs\Startup`.

#### Registry

Add a String value containing the path to the executable in the registry key `HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Run`.

#### Task Scheduler

Create a basic task triggered when the computer starts or when you log in with action set to start a program and browse to select the executable.

## Options

The executable will try to parse its first argument, if any, as an unsigned 64-bit integer and treat it as the duration in seconds to wait between two keyboard inputs.

For example:

```bash
nosleep.exe 120
```
