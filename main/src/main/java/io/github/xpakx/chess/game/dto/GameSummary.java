package io.github.xpakx.chess.game.dto;

import io.github.xpakx.chess.game.AIType;
import io.github.xpakx.chess.game.Game;
import io.github.xpakx.chess.game.GameStatus;
import io.github.xpakx.chess.game.GameType;
import lombok.Getter;
import lombok.Setter;

import java.util.Arrays;

@Getter
@Setter
public class GameSummary {
    private Long id;
    private Field[][] currentState;
    private Integer lastMoveRow;
    private Integer lastMoveColumn;
    private GameType type;
    private AIType aiType;

    private boolean finished;
    private boolean won;
    private boolean lost;
    private boolean drawn;

    private String username1;
    private String username2;
    private boolean userStarts;

    private boolean myTurn;

    public static GameSummary of(Game game, String requester) {
        var summary = new GameSummary();
        summary.setId(game.getId());
        summary.setCurrentState(stringToBoard(game.getCurrentState()));
        summary.setType(game.getType());
        summary.setAiType(game.getAiType());
        summary.setFinished(game.getStatus() != GameStatus.NotFinished);
        summary.setWon(game.getStatus() == GameStatus.Won);
        summary.setLost(game.getStatus() == GameStatus.Lost);
        summary.setDrawn(game.getStatus() == GameStatus.Drawn);
        summary.setUsername1(game.getUser().getUsername());
        summary.setUsername2(
                game.getOpponent() != null ? game.getOpponent().getUsername() : "AI"
        );
        if (requester.equals(summary.getUsername1())) {
            summary.setMyTurn(game.isUserTurn());
        } else if (requester.equals(summary.getUsername2())) {
            summary.setMyTurn(!game.isUserTurn());
        }
        summary.setUserStarts(game.isUserStarts());
        return summary;
    }

    private static Field[][] stringToBoard(String str) {
        String[] list = str.split(" ");
        if(list.length == 0) {
            return new Field[0][0];
        }
        String[] board = list[0].split("/");
        return Arrays.stream(board)
                .map(GameSummary::stringToRank)
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
