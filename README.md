# Fool's Paradise

My submission for the WASM-4 Game Jam 2022.

## Ideas for Improvement

* enemies that shoot diagonally
* powerups (spreader)
* add variety to bullet shots
* scores
* logo

## Big feature idea: daynight cycle

* day is less intense, crimson theme, all powerups given here
* night is a short, high intensity round where say, the speed of everything is doubled.  You use the powerups from the daytime to survive
* requires abstraction of the color theme (not hard)
* an indication of "rounds"

Memory issues still?  
* Make your own int -> string function and actually remove the fmt code (saves 3kb)
* remove getters, just make mutable reference calls all the time
* simplify xpos and ypos to just pos -> (u32, u32)
