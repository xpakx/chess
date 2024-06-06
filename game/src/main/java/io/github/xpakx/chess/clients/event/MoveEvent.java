package io.github.xpakx.chess.clients.event;

import lombok.Getter;
import lombok.Setter;

@Getter
@Setter
public class MoveEvent {
    private String gameState;
    private Long gameId;
    private String move;
    private Integer nonCapturingMoves;
}
