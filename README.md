# Connect4Neat

The following app is a machine learning neural agent that aims to solve the game
of connect 4 using the NEAT algorithm and MCTS like the AlphaZero agent that was developed by google for the game of chess.

## Theory

Contains two neural networks:

1. Value neural network
2. Policy neural network

For more information of how they work and how they are meant to interact
have a look at the reference <https://joshvarty.github.io/AlphaZero/>

## Description

How is it structured

1. Game mode
   * Needs to include as visual modes
     * CLI
     * GUI
   * Gameplay modes
     * PVP
     * PVE
       * Contains a random agent as easy bot
       * Different modes of depth plus some random play for difficulty. Use Bernoulli distribution and the level of randomness
   * Game flow
![Game](assets/game_flow.PNG)
2. Train mode
   1. Overview: ![Train overview diagram](assets/overview_training.png)
   2. Train flow: ![Diagram](assets/flow.png)

API interaction:
![API interaction](assets/api_iteraction.png)

## Possible setbacks

Dual training could confuse NEAT and lead to unwanted results. Careful inspection of the algorithm
or trial and error might be needed.
In case neat fails:

1. Reduce networks to one (First attempt this)
   1. Either by changing the fundamental algorithm
   2. One network serves as a carrier for both
      1. Neat will evolve both
2. Change algorithm

## References

* <https://joshvarty.github.io/AlphaZero/>
