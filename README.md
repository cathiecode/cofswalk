# Cofswalk

## Work in progress
This is WIP project.

## What
An experimental implementation of recursive directory traversal.

## Why
To make full system backup software, we need to iterate over thouthands and millions of files. However, if you want to use that in personal computer, it may not complete before it get turned off. So you need to save progression for the next worktime. Unfortunately existing implementations does not support that. That's why I made this.

## Roadmap
- [x] Basic functionality
- [ ] Error handling
- [ ] Symbolic link support
- [ ] Hard link (Cyclic directory) support
- [ ] Cross platform
- [ ] Resume in removed directory
- [ ] Optimization
- [ ] Stabilize API(1.0)
