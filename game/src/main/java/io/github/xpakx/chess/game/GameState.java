package io.github.xpakx.chess.game;

import io.github.xpakx.chess.game.dto.Color;
import lombok.Getter;
import lombok.Setter;
import org.springframework.data.redis.core.RedisHash;

import java.io.Serializable;
import java.util.Optional;

@Getter
@Setter
@RedisHash
public class GameState implements Serializable {
    private Long id;
    private boolean finished;
    private boolean won;
    private boolean drawn;
    private boolean lost;

    private String currentState;

    private String username1;
    private String username2;
    private boolean user2AI;
    private AIType aiType;

    private boolean firstUserStarts;
    private boolean firstUserTurn;
    private Integer nonCaptureMoves;

    private boolean blocked;

    public boolean isSecondUserTurn() {
        return !isFirstUserTurn();
    }

    public boolean isUserInGame(String username) {
        return username.equals(username1) ||
                (!user2AI && username.equals(username2));
    }

    public void nextPlayer() {
        firstUserTurn = !firstUserTurn;
    }

    public String getCurrentPlayer() {
        if (isFirstUserTurn()) {
            return username1;
        }
        return username2;
    }

    public boolean aiTurn() {
        return user2AI && isSecondUserTurn();
    }

    public Optional<String> getWinner() {
        if (!won) {
            return Optional.empty();
        }
        var winner = getCurrentPlayer();
        return Optional.of(winner != null ? winner : "AI");
    }

    public Color getColor() {
        if(firstUserStarts) {
            return firstUserTurn ? Color.White : Color.Black;
        }
        return firstUserTurn ? Color.Black : Color.White;
    }
}

