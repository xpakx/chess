package io.github.xpakx.chess.game;

import io.github.xpakx.chess.game.dto.UpdateEvent;
import io.github.xpakx.chess.game.error.GameNotFoundException;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
@RequiredArgsConstructor
public class MoveService {
    private final MoveRepository moveRepository;
    private final GameRepository gameRepository;

    public void saveMove(UpdateEvent event) {
        var gameOpt = gameRepository.findWithUsersById(event.getGameId());
        if (gameOpt.isEmpty()) {
            return;
        }
        var game = gameOpt.get();
        var move = new Move();
        move.setGame(gameRepository.getReferenceById(event.getGameId()));
        move.setTimestamp(event.getTimestamp());
        move.setCurrentState(event.getCurrentState());
        move.setMove(event.getMove());
        if (event.isUserTurn()) {
            move.setUser(game.getUser());
        } else {
            move.setUser(game.getOpponent());
        }
        moveRepository.save(move);
    }

    public List<Move> getMoveHistory(Long gameId) {
        if (!gameRepository.existsById(gameId)) {
            throw new GameNotFoundException();
        }
        return moveRepository.findByGameIdOrderByTimestampAsc(gameId);
    }

}
