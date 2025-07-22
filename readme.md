# Sampler
A music sampling app made by Ayaan Irshad

## How it works
### Crates
[Rodio](https://github.com/RustAudio/rodio/tree/c0fa763ae0a06553dafbf612a84001912288cd13): Used for audio playback

[Random-Color](https://crates.io/crates/random_color) - Generates a random color

[Tokio](https://github.com/tokio-rs/tokio) - Used for multithreading/concurrency

[Youtube_Dl](https://github.com/GyrosOfWar/youtube-dl-rs) - Simulates the cmd line tool to download audio from youtube

[Audio-Visualizer](https://github.com/phip1611/audio-visualizer) - Used as a submodule (I made some modifications) to get the waveform image displaying. My fork is [here](https://github.com/Yodaman07/audio-visualizer/tree/68f50e66e0f6cbc4fcc186d28d5263d9cdea34e9)

(See the full list in `Cargo.toml`)

## How to use it

1. Paste the link to a youtube song or upload it locally following the dialog on the left
2. Once imported, you will be able to hear the audio play, a waveform will generate and appear shortly
3. Click on `new chop` and you can select and drag it anywhere on the bottom timeline
4. You should see a tint in the screen color. Then play the original song and select the times for the start and end markers to be placed. The song will loop around those.
5. Once all of your chops are ready, press the play button on the bottom and you will hear your chops play (as of 7/21/2025 the order doesn't change anything, will be fixed soon)



## Goal (MVP)
The goal of the sampler is to allow for audio tracks from youtube and your own machine to be imported, and chopped/arranged in a manner that is simple for anyone to pick and also exciting to play around with different chops with effects and more

## Current State
As of 7/21/2025, the sampler supports importing music locally or via youtube and chopping them on one timeline, and playing it back. Effects have not yet been added.

## Possible future additions
Along with effects, multiple chop timelines, importing multiple songs, as well as possible midi integration is in consideration.