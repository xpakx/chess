package io.github.xpakx.chess.clients.event;

import io.github.xpakx.chess.game.dto.Color;
import lombok.Getter;
import lombok.Setter;

@Getter
@Setter
public class MoveEvent {
    private String gameState;
    private Long gameId;
    private String move;
    private Integer nonCapturingMoves;
    private Color color;
}
