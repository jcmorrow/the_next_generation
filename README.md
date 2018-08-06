ToDos
---

Interfaces to implement:

- Cards
  - Card types?
- Hand
- Game
- Player
- Board
- Player Area

Things Games Have
---

Games have a marketplace (which is just a collection of cards)
Games have a marketplace deck (which is just a collection of cards)
Games have an active player (which is just a single player)
Games have opponents (which is a collection of players)

What does a Game Turn look like?

A game turn is when both players take a turn. Each player taking turn should
return two things:

An Option result which indicates if the game is done. A game state (If the game
is over this can be null?).

A player turn has

