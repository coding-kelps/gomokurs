# Gomokurs Protocol over TCP

## Overview

This document details the protocol used to communicate actions between the manager and the players over TCP.
This protocol has been heavily inspired by the [Gomoku AI Protocol](https://plastovicka.github.io/protocl2en.htm) used by the [Plastovicka's Piskvork software](https://github.com/plastovicka/Piskvork).

## Actions

The requests and response send by the manager to a player and by the player to the manager are referred as "actions".  
You find in the following sections the complete list of actions that can be taken by the manager and the players.  
Each actions is identified by its binary identifier which is systematically send in first.

### Manager's Actions

#### PROTOCOL COMPATIBLE

```
0x00
```

#### START

##### Summary

The START action is used to communicate to a player the initial configuration of the board that it should init.  
The manager will then expect the player to respond with a [READY action](#ready).

##### Content

```
0x01 [START_SIZE]
```

- [START_SIZE],     1 byte big endian unsigned integer:
    The size of the gomoku square board in both X and Y.

#### TURN

##### Summary

The TURN action is used to communicate to a player its opponent turn.
The manager will then expect the player to respond with a [PLAY action](#play).

##### Content

```
0x02 [X] [Y]
```

- [X],              1 byte big endian unsigned integer:
    The position of the opponent play in the gomoku board on the X axis.
- [Y],              1 byte big endian unsigned integer:
    The position of the opponent play in the gomoku board on the Y axis.

#### BEGIN

##### Summary

The BEGIN action is used to communicate to a player that he is the first player to play.
The manager will then expect the player to respond with a [PLAY action](#play).

##### Content

```
0x03
```

#### BOARD

##### Summary

The BOARD action is used to communicate to a player an history of turns that he should register to the board.
The manager will then expect the player to respond with a [PLAY action](#play).

##### Content

```
0x04 [NB_TURN] [TURNS: [[X] [Y] [FIELD]], ...]
```

- [NB_TURN],        4 bytes big endian unsigned integer:
- [TURNS],          4 bytes big endian unsigned integer:
    - [X],          1 byte big endian unsigned integer:
    - [Y],          1 byte big endian unsigned integer:
    - [FIELD]       1 byte big endian unsigned integer:

#### INFO

##### Summary

The INFO action is used to communicate an information as a string to a player.
The manager will then expect the player to respond with a [PLAY action](#play).

##### Content

```
0x05 [PAYLOAD_SIZE] [PAYLOAD]
```

- [PAYLOAD_SIZE],   4 bytes big endian unsigned integer:
- [PAYLOAD],        N bytes UTF-8 string:

#### END

##### Summary

The END is used to communicate the termination of the session to the player.
Following this action, the manager will not expect anymore actions from the player.

##### Content

```
0x06
```

#### ABOUT

##### Summary

The ABOUT action is used to requests to a player its metadata.
The manager will then expect the player to respond with a [PLAYER DESCRIPTION action](#player-description).

##### Content

```
0x07
```

#### UNKNOWN

##### Summary

##### Content

```
0x08 [PAYLOAD_SIZE] [PAYLOAD]
```

- [PAYLOAD_SIZE],   4 bytes big endian unsigned integer:
- [PAYLOAD],        N bytes UTF-8 string:

#### ERROR

##### Summary

##### Content

```
0x09 [PAYLOAD_SIZE] [PAYLOAD]
```

- [PAYLOAD_SIZE],   4 bytes big endian unsigned integer:
- [PAYLOAD],        N bytes UTF-8 string:

### Player's Actions

#### PROTOCOL VERSION

##### Summary

##### Content

```
0x0A [PAYLOAD_SIZE] [PAYLOAD]
```

- [PAYLOAD_SIZE],   4 bytes big endian unsigned integer:
- [PAYLOAD],        N bytes UTF-8 string:

#### READY

##### Summary

##### Content

```
0x0B
```

#### PLAY

##### Summary

##### Content

```
0x0C [X] [Y]
```

- [X],              1 byte big endian unsigned integer: The 
- [Y],              1 byte big endian unsigned integer: The

#### PLAYER DESCRIPTION

##### Summary

##### Content

```
0x0D [PAYLOAD_SIZE] [PAYLOAD]
```

- [PAYLOAD_SIZE],   4 bytes big endian unsigned integer:
- [PAYLOAD],        N bytes UTF-8 string:

#### UNKNOWN

##### Summary

##### Content

```
0x0E [PAYLOAD_SIZE] [PAYLOAD]
```

- [PAYLOAD_SIZE],   4 bytes big endian unsigned integer:
- [PAYLOAD],        N bytes UTF-8 string:

#### ERROR

##### Summary

##### Content

```
0x0F [PAYLOAD_SIZE] [PAYLOAD]
```

- [PAYLOAD_SIZE],   4 bytes big endian unsigned integer:
- [PAYLOAD],        N bytes UTF-8 string:

#### MESSAGE

##### Summary

##### Content

```
0x10 [PAYLOAD_SIZE] [PAYLOAD]
```

- [PAYLOAD_SIZE],   4 bytes big endian unsigned integer:
- [PAYLOAD],        N bytes UTF-8 string:

#### DEBUG

##### Summary

##### Content

```
0x11 [PAYLOAD_SIZE] [PAYLOAD]
```

- [PAYLOAD_SIZE],   4 bytes big endian unsigned integer:
- [PAYLOAD],        N bytes UTF-8 string:

#### SUGGESTION

##### Summary

##### Content

```
0x12 [X] [Y]
```

- [X],              1 byte big endian unsigned integer: The 
- [Y],              1 byte big endian unsigned integer: The
