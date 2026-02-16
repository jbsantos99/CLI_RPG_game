# Rust CLI RPG Game

## What is this about?

This started as a way to learn Rust and low level concepts by building, using no AI ( like the Incas ). The initial idea was to build a simple CLI, RPG, turn based game, but ideas kept coming and I was having soo much fun adding them, that I just kept going...

For names, mechanics and style I took inspiration from many games, including, but not restricted to: The Division, Stardew Valley and GoodGame Gangster. Feels like I distilled the things I loved the most among those games and putted here.

Also, this is my first time using Neovim for a full project, from start to finish. Deleted VSCode and Cursor and finilly decide to use it for real, to see if I like it or not, instead of fulling myself with VIM Motions. 

(I'm using Neovim btw)

### Game Design

The game consists of a player fighting bosses and upgradint its gear. Killing the final boss will finish the game.

Player has a struct with total hp, damage, critical hit change, weapons, defense, etc.

By killing bosses, the player will make money, with money he/she can buy better gear and continue.


## Implemented

- Critical hit mechanic
- Make is so hit value shows after boss name
- Implement a money system, boss pays after death ( after defeated, boss pays half )

## Todos

#### Code Improvements
- Change check_saves to return only booleans and handle internally if an error occours, maybe loop until theres no longer error.
- Make the player healthbar be alays a percentage of the of the boss, if the boss has less, invert
- Change the color of the bar while the health goes down ( and have a stronger colors of hp bars for huge number, like purple)
- add chance of blocking the attack
- add a little delay when the hit is going to be critical

- This seems to be wrong with an empty function
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


#### Game features
- Add a vampire mechanic: part of every hit you give goes back to you ( some bosses can have this too ). [ maybe you can you gain it by defeating the boss]
- create a "select path", for gaining xp levels, where you can choose between a couple buff options ( with debuffs maybe? )
- create a function to raize coins and ex ( bribe ), to have the max of 10h of bribe
- Create xp system

## Low priority
- Put the bars in opposite directions
- when comparing boss x player stats, display more nicelly the struct
- create boss and player visuals?
- add a typing animation intead of just printing stuff
- add some sound effects





