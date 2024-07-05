package io.github.xpakx.chess.game.dto;

import com.fasterxml.jackson.annotation.JsonInclude;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;

@Getter
@Setter
@AllArgsConstructor
@NoArgsConstructor
public class MoveMessage {
    private String player;
    private String move;
    private boolean legal;

    @JsonInclude(JsonInclude.Include.NON_NULL)
    private String message;

    private boolean finished;
    private boolean won;

    @JsonInclude(JsonInclude.Include.NON_NULL)
    private String winner;
    private Color color;

    public static MoveMessage of(String move, String username, Color color) {
        return new MoveMessage(
                username,
                move,
                true,
                null,
                false,
                false,
                null,
                color
        );
    }

    public static MoveMessage rejected(String move, String username, Color color, String msg) {
        var moveMessage = of(move, username, color);
        moveMessage.setMessage(msg);
        moveMessage.setLegal(false);
        return moveMessage;
    }
}
