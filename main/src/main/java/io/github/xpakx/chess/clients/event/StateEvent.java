package io.github.xpakx.chess.clients.event;

import io.github.xpakx.chess.game.AIType;
import lombok.Getter;
import lombok.Setter;

@Getter
@Setter
public class StateEvent {
    private Long id;
    private boolean finished;
    private String currentState;

    private String username1;
    private String username2;
    private boolean user2AI;
    private AIType aiType;

    private boolean firstUserStarts;
    private boolean firstUserTurn;
    private boolean error;
    private String errorMessage;
    private Integer nonCaptureMoves;
}
