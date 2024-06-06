package io.github.xpakx.chess.game.dto;

import lombok.Getter;
import lombok.Setter;

@Getter
@Setter
public class EngineMoveEvent {
    Long gameId;
    boolean legal;
    String move;
    boolean finished;
    String newState;
}
