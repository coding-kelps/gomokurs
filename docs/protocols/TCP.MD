# Gomokurs Protocol over TCP - Version 0.2.0

## Overview

This document describes the protocol used to exchange actions between the **manager** and the **players** over TCP.  
It is heavily inspired by the [Gomoku AI Protocol](https://plastovicka.github.io/protocl2en.htm) used by [Plastovicka's Piskvork software](https://github.com/plastovicka/Piskvork).

## Actions

All requests and responses exchanged between the manager and a player are referred to as **actions**.  
The following sections list all possible actions that can be performed by either the manager or the players.  
Each action is identified by its binary identifier, which is always sent first.

---

## Manager’s Actions

### PROTOCOL COMPATIBLE

#### Summary

The **PROTOCOL COMPATIBLE** action is sent by the manager in response to a player’s [PROTOCOL VERSION](#protocol-version) action. It informs the player that their protocol versions are compatible and that the session can proceed.

#### Content

```
0x00
```

---

### START

#### Summary

The **START** action instructs a player to initialize a board of a given size.  
After sending **START**, the manager expects the player to reply with a [READY](#ready) action.

#### Content

```
0x01 [START_SIZE]
```

- **[START_SIZE]** (1-byte big-endian unsigned integer)  
  The size of the Gomoku board (width and height).

---

### RESTART

#### Summary

The **RESTART** action instructs a player to initialize a board of a given size with same configuration as the previous game.  
After sending **RESTART**, the manager expects the player to reply with a [READY](#ready) action.

#### Content

```
0x02
```

---

### TURN

#### Summary

The **TURN** action communicates the opponent’s last move to the player.  
After sending **TURN**, the manager expects the player to reply with a [PLAY](#play) action.

#### Content

```
0x03 [X] [Y]
```

- **[X]** (1-byte big-endian unsigned integer)  
  The X-position of the opponent’s move.
- **[Y]** (1-byte big-endian unsigned integer)  
  The Y-position of the opponent’s move.

---

### BEGIN

#### Summary

The **BEGIN** action notifies a player that it is the first to move.  
After sending **BEGIN**, the manager expects the player to reply with a [PLAY](#play) action.

#### Content

```
0x04
```

---

### BOARD

#### Summary

The **BOARD** action provides a list of past moves (an initial history) that the player should register on the board.  
After sending **BOARD**, the manager expects the player to reply with a [PLAY](#play) action.

#### Content

```
0x05 [NB_TURN] [TURNS: [[X] [Y] [FIELD]], ...]
```

- **[NB_TURN]** (4-byte big-endian unsigned integer)  
  The number of turns provided in this **BOARD** action.
- **[TURNS]**  
  An array of **NB_TURN** records, each consisting of:
  - **[X]** (1-byte big-endian unsigned integer)  
    The X-position of the move.
  - **[Y]** (1-byte big-endian unsigned integer)  
    The Y-position of the move.
  - **[FIELD]** (1-byte big-endian unsigned integer)  
    Indicates which player made the move, relative to the recipient:
    - `1`: The recipient’s own move
    - `2`: The opponent’s move

---

### INFO

#### Summary

The **INFO** action transmits an informational message (as a string) to the player.  
After sending **INFO**, the manager expects the player to reply with a [PLAY](#play) action.

> **Note:** Due to standard I/O compatibility, the information string must not contain any newline characters.

#### Content

```
0x06 [PAYLOAD_SIZE] [PAYLOAD]
```

- **[PAYLOAD_SIZE]** (4-byte big-endian unsigned integer)  
  The size in bytes of the payload.
- **[PAYLOAD]** (N-byte UTF-8 string)  
  The informational message.

---

### RESULT

#### Summary

The **RESULT** action provides the game result to the player.

#### Content

```
0x07 [RESULT]
```

- **[RESULT]** (1-byte big-endian unsigned integer)  
  Indicates the outcome of the player's game:
  - `0`: The game ended in a draw  
  - `1`: The player won the game  
  - `2`: The player lost the game  

---

### END

#### Summary

The **END** action indicates that the session is terminated.  
After **END** is sent, the manager does not expect any further actions from the player.

#### Content

```
0x08
```

---

### ABOUT

#### Summary

The **ABOUT** action requests metadata from a player.  
After sending **ABOUT**, the manager expects the player to reply with a [PLAYER DESCRIPTION](#player-description) action.

#### Content

```
0x09
```

---

### UNKNOWN

#### Summary

The **UNKNOWN** action is sent by the manager if it does not recognize the player’s last action. Immediately after sending **UNKNOWN**, the manager terminates the session.

> **Note:** The error message must not contain any newline characters.

#### Content

```
0x0A [PAYLOAD_SIZE] [PAYLOAD]
```

- **[PAYLOAD_SIZE]** (4-byte big-endian unsigned integer)  
  The size in bytes of the payload.
- **[PAYLOAD]** (N-byte UTF-8 string)  
  The error message.

---

### ERROR

#### Summary

The **ERROR** action is sent by the manager if there is an error in the player’s last action. Immediately after sending **ERROR**, the manager terminates the session.

> **Note:** The error message must not contain any newline characters.

#### Content

```
0x0B [PAYLOAD_SIZE] [PAYLOAD]
```

- **[PAYLOAD_SIZE]** (4-byte big-endian unsigned integer)  
  The size in bytes of the payload.
- **[PAYLOAD]** (N-byte UTF-8 string)  
  The error message.

---

## Player’s Actions

### PROTOCOL VERSION

#### Summary

The **PROTOCOL VERSION** action is sent by the player to communicate its protocol version to the manager. This step ensures compatibility.  
The version is sent as a simple string.  
After sending **PROTOCOL VERSION**, the player expects the manager to respond with either [PROTOCOL COMPATIBLE](#protocol-compatible) (if the versions are compatible) or [ERROR](#error) (if they are not).  

The **PROTOCOL VERSION** action is expected to be the first action a player sends so that both the manager and the player can confirm protocol compatibility before proceeding.

#### Content

```
0x0C [PAYLOAD_SIZE] [PAYLOAD]
```

- **[PAYLOAD_SIZE]** (4-byte big-endian unsigned integer)  
  The size in bytes of the payload.
- **[PAYLOAD]** (N-byte UTF-8 string)  
  The protocol version string.

---

### READY

#### Summary

The **READY** action is sent by the player in response to a [START](#start) action to confirm that it has initialized the game state and is ready to play.

#### Content

```
0x0D
```

---

### PLAY

#### Summary

The **PLAY** action communicates the player’s move to the manager.  
It is sent in response to [BEGIN](#begin), [TURN](#turn), or [BOARD](#board) actions.

#### Content

```
0x0E [X] [Y]
```

- **[X]** (1-byte big-endian unsigned integer)  
  The X-position of the player’s chosen move.
- **[Y]** (1-byte big-endian unsigned integer)  
  The Y-position of the player’s chosen move.

---

### PLAYER DESCRIPTION

#### Summary

The **PLAYER DESCRIPTION** action is sent by the player in response to the manager’s [ABOUT](#about) action, providing metadata about the player.

The metadata is provided as a list of key-value pairs, separated by commas (and optional spaces), for example:

```
name="SomeBrain", version="1.0", author="Nymand", country="USA"
```

> **Note:** The metadata must not contain any newline characters.

#### Content

```
0x0F [PAYLOAD_SIZE] [PAYLOAD]
```

- **[PAYLOAD_SIZE]** (4-byte big-endian unsigned integer)  
  The size in bytes of the payload.
- **[PAYLOAD]** (N-byte UTF-8 string)  
  The player’s metadata.

---

### UNKNOWN

#### Summary

The **UNKNOWN** action is sent by the player if it does not recognize the manager’s last action. Immediately after sending **UNKNOWN**, the player terminates the session.

> **Note:** The error message must not contain any newline characters.

#### Content

```
0x10 [PAYLOAD_SIZE] [PAYLOAD]
```

- **[PAYLOAD_SIZE]** (4-byte big-endian unsigned integer)  
  The size in bytes of the payload.
- **[PAYLOAD]** (N-byte UTF-8 string)  
  The error message.

---

### ERROR

#### Summary

The **ERROR** action is sent by the player if there is an error in the manager’s last action. Immediately after sending **ERROR**, the player terminates the session.

> **Note:** The error message must not contain any newline characters.

#### Content

```
0x11 [PAYLOAD_SIZE] [PAYLOAD]
```

- **[PAYLOAD_SIZE]** (4-byte big-endian unsigned integer)  
  The size in bytes of the payload.
- **[PAYLOAD]** (N-byte UTF-8 string)  
  The error message.

---

### MESSAGE

#### Summary

> **Note:** The **MESSAGE** action is currently **not supported** by the Gomokurs manager. It is included here only for compatibility with the [Gomoku AI Protocol](https://plastovicka.github.io/protocl2en.htm).

The **MESSAGE** action is used by the player to communicate a message intended for the end-user of the manager.

#### Content

```
0x12 [PAYLOAD_SIZE] [PAYLOAD]
```

- **[PAYLOAD_SIZE]** (4-byte big-endian unsigned integer)  
  The size in bytes of the payload.
- **[PAYLOAD]** (N-byte UTF-8 string)  
  The message.

---

### DEBUG

#### Summary

The **DEBUG** action is used by the player to send a debugging message to the manager. The manager is expected to log this message but not necessarily display it to the user.

#### Content

```
0x13 [PAYLOAD_SIZE] [PAYLOAD]
```

- **[PAYLOAD_SIZE]** (4-byte big-endian unsigned integer)  
  The size in bytes of the payload.
- **[PAYLOAD]** (N-byte UTF-8 string)  
  The debug message.

---

### SUGGESTION

#### Summary

> **Note:** The **SUGGESTION** action is currently **not supported** by the Gomokurs manager. It is included here only for compatibility with the [Gomoku AI Protocol](https://plastovicka.github.io/protocl2en.htm).

The **SUGGESTION** action is used by the player to suggest a move to the user of the Gomokurs server.

#### Content

```
0x14 [X] [Y]
```

- **[X]** (1-byte big-endian unsigned integer)  
  The X-position of the suggested move.
- **[Y]** (1-byte big-endian unsigned integer)  
  The Y-position of the suggested move.
