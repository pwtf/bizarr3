![BIZARR3](logo.png)

> **Bizarr3** /bɪˈzɑː(ɹ)/ **1** _adj_. (also *b1zarr3*), *leetspeak* for *bizarre*. **2** _n._ A set of tools to manipulate the _1nsane_ game data.


# Prologue
This work started as an exercise in reverse engineering and as a fulfilling piece of calm to a childly obsession to tear toys apart to see what's inside that never vent away.

The target of choice is an off-roading racing game from the past century — 1nsane — that made revolution in my perception of racing games with it's graphics and realism, but somehow been greeted coldly by the gamers of the era and quickly became forgotten.

The ultimate goal is to sharpen my decaying skills (reverse-engineering, for example), to catch up with the technology (say, by making a WebGL model viewer/editor) and to learn something new (the Rust programming language). Though not the main objective, other positive side-effects like understanding the physics engine, rendering pipeline and networking protocols are highly anticipated and welcome.

# File Formats
## IDF
All the game assets are stored in the [.idf](idf_format.md) files.
## XFF
Most of the text resources (`.cfg`, `.dat`, `.scn`, `.vht`) are encoded with this format.
