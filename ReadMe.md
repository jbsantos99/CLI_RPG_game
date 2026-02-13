# Rust CLI RPG Game

## What is this game?

This is personal project intended to practice the basics of Rust and create a nice little terminal based game.



### Game Design

The game consists of a player fighting bosses and upgradint its gear. Killing the final boss will finish the game.

Player has a struct with total hp, damage, critical hit change, weapons, defense, etc.

By killing bosses, the player will make money, with money he/she can buy better gear and continue.


## Implemented

- Critical hit mechanic
- Make is so hit value shows after boss name
- Implement a money system, boss pays after death ( after defeated, boss pays half )
## Todos

- Change check_saves to return only booleans and handle internally if an error occours, maybe loop until theres no longer error.
- Make the player healthbar be alays a percentage of the of the boss, if the boss has less, invert
- Change the color of the bar while the health goes down
- Create xp system
- create a  "select path", for gaining xp levels
- create a function to raize coins and ex ( bribe ), to have the max of 10h of bribe
- maybe add some sound effects
- add chance of blocking the attack
- add a little delay when the hit is going to be critical

- This seams to be wrong with an empty function


```
        Ok(true) => {}
        Ok(false) => {
            println!("Bosses save files not detected!");
            generate_bosses();
        }

        Err(err) => {
            println!("Error checking bosses save files {}", err)
        }
    }

```




## Problems
- Hard time aligning the health bars in opposite directions





