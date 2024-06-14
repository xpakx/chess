package io.github.xpakx.chess.clients.event;

import com.fasterxml.jackson.annotation.JsonInclude;
import io.github.xpakx.chess.game.AIType;
import io.github.xpakx.chess.game.dto.Color;
import lombok.Getter;
import lombok.Setter;

@Getter
@Setter
public class AIEvent {
    private Long gameId;
    private String gameState;
    private Integer nonCapturingMoves;
    private AIType type;
    private Color color;
}
