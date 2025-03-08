# cantometria

🎤 A rust library & TUI Application to grade human singing accuracy.

> [!IMPORTANT]
> This project is developed under the **01204371 Transform Techniques in Signal Processing** course of **Department of Computer Engineering**, **Faculity of Engineering**, **Kasetsart University**.
> **Project Developers**: *Kritchanat Thanapiphatsiri (6610501955)*

## Installation
1. Install [rust](https://www.rust-lang.org/tools/install).
2. Once finished, open the terminal, and run these commands:
```sh
git clone https://github.com/krtchnt/cantometria
cd cantometria
cargo build --release
```

## Usage
To launch the program, run the program `cantometria_tui` in the folder  `target/release/`, or run `cd cantometria && cargo run --release` in the terminal.
Follow the instructions on the screen, and the program will output the singing accuracy at the end.

To add more singing recordings, add them to the folder `test`. Make sure they are a WAV (`.wav`) file.

To add more songs or melodies, add them to the folder `midi`. Make sure they are a MIDI (`.mid`) file.
To find the files, many popular songs are readily available on the internet as MIDI and a simple web search will most likely obtain you what you want.

> [!NOTE]
> For the most accurate grading, ensure the recorded audio is clear of any noises and other sounds that are not the singing voice to be graded.
> The recording will start being graded immediately as soon as the first note on the MIDI file. As a result, it is better to count down before recording starts.
>
> The selected MIDI file must only have one track/instrument, which will be the sung melodies. The program would refuse to run otherwise.

## Test Media Sources
|Track Title|Original Melody|Source Recording|
|-|-|-|
|`test`|Control Melody[^1]|Developer Recording[^2]|
|`tetris[-2]`|*Tetris - Theme A*|Developer Recording[^2]|
|`bite-*`|[*Sān-Z - BITE!*](https://open.spotify.com/album/2CtVCZzIc9ujfohxzF5WwK)|[*Zenless Zone Zero*](https://zenless.hoyoverse.com/en-us/) (Ellen Joe's Chinese, Japanese and Korean Voice-over)|

[^1]: A continuous-time note signal of an ascending C major scale from C4 to C5, each note sustained for 500 ms.
[^2]: Self-recorded and sung by the project developers.
