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

    private Field[][] state;
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

    private static Field[][] stringToBoard(String str) {
        String[] list = str.split(" ");
        if(list.length == 0) {
            return new Field[0][0];
        }
        String[] board = list[0].split("/");
        return Arrays.stream(board)
                .map(GameMessage::stringToRank)
                .toArray(Field[][]::new);
    }

    private static Field[] stringToRank(String rank) {
        final int DIMENSION = 8;
        var result = new Field[DIMENSION];
        int position = 0;
        for(int i = 0; i < rank.length() && position < DIMENSION; i++) {
            char ch = rank.charAt(i);
            if(!Character.isDigit(ch)) {
                result[position++] = charToSymbol(ch);
                continue;
            }
            int emptyFields = Character.getNumericValue(ch);
            Arrays.fill(result, position, Math.min(position + emptyFields, DIMENSION), Field.Empty);
            position += emptyFields;
        }
        return result;
    }

    private static Field charToSymbol(char c) {
        return switch (c) {
            case 'P' -> Field.WhitePawn;
            case 'N' -> Field.WhiteKnight;
            case 'B' -> Field.WhiteBishop;
            case 'R' -> Field.WhiteRook;
            case 'Q' -> Field.WhiteQueen;
            case 'K' -> Field.WhiteKing;
            case 'p' -> Field.BlackPawn;
            case 'n' -> Field.BlackKnight;
            case 'b' -> Field.BlackBishop;
            case 'r' -> Field.BlackRook;
            case 'q' -> Field.BlackQueen;
            case 'k' -> Field.BlackKing;
            default -> Field.Empty;
        };
    }
}
