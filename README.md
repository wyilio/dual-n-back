# Dual-N Back

Dual-N Back is a mental game designed to challenge your working memory using two types of cues: visual and auditory.

In each session, a 3x3 grid is displayed to players. During each trial, a specific square in the grid lights up while an alphabetic letter is simultaneously played. The player's objective is to determine if the same square and/or sound appeared exactly N rounds prior.
The concept of the game was first introduced by Jaeggi et al. in a [2008 PNAS study](https://www.pnas.org/doi/10.1073/pnas.0801268105).

If you'd like to read more, check out this comprehensive [article](https://gwern.net/dnb-faq) by Gwern on the subject. 

This project owes its existence to [Brain Workshop](https://brainworkshop.sourceforge.net/). Down the line, it'd be nice to incorporate more of Brain Workshop's features.

# Implementation Details

This project uses Rust, Bevy, and egui. If you have proposed changes, feel free to write an issue. 

# Notes

- To exit a screen, hit <esc>
- If you want to adjust your level, number of trials, or game thresholds, you can use the settings panel.

# License

This project is dual-licensed under Apache 2.0 and MIT.
