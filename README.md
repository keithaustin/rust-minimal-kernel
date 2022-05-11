# A minimal OS kernel in Rust

The goal of this project is to write a small, but functional OS kernel in pure Rust. 

Currently working:
- It actually boots! Technically it should boot on bare metal, however I have only test on a VM currently.
- Text can be outputted to the screen via a VGA buffer.

Currently not working:
- Basically everything else. Currently working on some boring stuff to make the code more testable.
- Currently only supports legacy BIOS boot, because I have no idea how to do UEFI boot. There are UEFI crates but I would like to learn how to do as much of this on my own as possible. I would actually like to learn more about the workings of the bootloader crate I'm using, that also feels like cheating...

Future goals:
- Supporting UEFI boot
- Supporting some other common CPU architectures such as ARM or MIPS.
- At the very least, a minimal shell. It doesn't have to be as feature-rich as UNIX, it just has to do some stuff.

This project has been very interesting so far, it's surprising how much harder it is to write code without relying on an underlying OS for common functionality. This is not really meant to be a functional, usable piece of software at any point, it is more of a learning project for me to familiarize myself with Rust and more systems-level software dev. 