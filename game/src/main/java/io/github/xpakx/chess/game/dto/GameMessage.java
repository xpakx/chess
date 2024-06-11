package io.github.xpakx.chess.game.dto;

import com.fasterxml.jackson.annotation.JsonInclude;
import io.github.xpakx.chess.game.GameState;
import lombok.Getter;
import lombok.Setter;

import java.util.Arrays;

@Getter
@Setter
public class GameMessage {
    private String username1;
    private String username2;
    private boolean ai;

    private String[][] state;
    private String currentPlayer;
    private boolean firstUserStarts;

    @JsonInclude(JsonInclude.Include.NON_NULL)
    private String error;

    public static GameMessage of(GameState game) {
        var msg = new GameMessage();
        msg.setUsername1(game.getUsername1());
        msg.setUsername2(game.getUsername2());
        msg.setAi(game.isUser2AI());
        msg.setState(stringToBoard(game.getCurrentState()));
        msg.setCurrentPlayer(game.isFirstUserTurn() ? game.getUsername1() : game.getUsername2());
        msg.setFirstUserStarts(game.isFirstUserStarts());
        return msg;
    }

    private static String[][] stringToBoard(String str) {
        String[] list = str.split(" ");
        if(list.length < 1) {
            return null;
        }
        String[] board = list[0].split("/");
        return Arrays.stream(board)
                .map(GameMessage::stringToRank)
                .toArray(String[][]::new);
    }

    private static String[] stringToRank(String rank) {
        final int DIMENSION = 8;
        var result = new String[DIMENSION];
        int position = 0;
        for(int i = 0; i < rank.length() && position < DIMENSION; i++) {
            char ch = rank.charAt(i);
            if(!Character.isDigit(ch)) {
                result[position++] = charToSymbol(ch);
                continue;
            }
            int emptyFields = Character.getNumericValue(ch);
            Arrays.fill(result, position, Math.min(position + emptyFields, DIMENSION), "Empty");
            position += emptyFields;
        }
        return result;
    }

    private static String charToSymbol(char c) {
        return switch (c) {
            case 'P' -> "WhitePawn";
            case 'N' -> "WhiteKnight";
            case 'B' -> "WhiteBishop";
            case 'R' -> "WhiteRook";
            case 'Q' -> "WhiteQueen";
            case 'K' -> "WhiteKing";
            case 'p' -> "BlackPawn";
            case 'n' -> "BlackKnight";
            case 'b' -> "BlackBishop";
            case 'r' -> "BlackRook";
            case 'q' -> "BlackQueen";
            case 'k' -> "BlackKing";
            default -> "Empty";
        };
    }
}
