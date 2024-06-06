package io.github.xpakx.chess.game.dto;

import com.fasterxml.jackson.annotation.JsonInclude;
import io.github.xpakx.chess.game.GameState;
import lombok.Getter;
import lombok.Setter;

@Getter
@Setter
public class GameMessage {
    private String username1;
    private String username2;
    private boolean ai;

    private String[][] state;
    private String currentPlayer;

    @JsonInclude(JsonInclude.Include.NON_NULL)
    private String error;

    public static GameMessage of(GameState game) {
        var msg = new GameMessage();
        msg.setUsername1(game.getUsername1());
        msg.setUsername2(game.getUsername2());
        msg.setAi(game.isUser2AI());
        msg.setState(stringToBoard(game.getCurrentState()));
        msg.setCurrentPlayer(game.isFirstUserTurn() ? game.getUsername1() : game.getUsername2());
        return msg;
    }

    private static String[][] stringToBoard(String str) {
        //TODO
        return null;
    }

    private static String charToSymbol(char c) {
        // TODO
        return "";
    }

}
