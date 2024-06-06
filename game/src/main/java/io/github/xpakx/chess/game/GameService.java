package io.github.xpakx.chess.game;

import io.github.xpakx.chess.clients.GamePublisher;
import io.github.xpakx.chess.clients.MovePublisher;
import io.github.xpakx.chess.clients.StatePublisher;
import io.github.xpakx.chess.game.dto.*;
import lombok.RequiredArgsConstructor;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.messaging.simp.SimpMessagingTemplate;
import org.springframework.stereotype.Service;

import java.util.Optional;

@Service
@RequiredArgsConstructor
public class GameService {
    private final SimpMessagingTemplate simpMessagingTemplate;
    private final GameRepository repository;
    private final MovePublisher movePublisher;
    private final GamePublisher gamePublisher;
    private final StatePublisher statePublisher;
    Logger logger = LoggerFactory.getLogger(GameService.class);

    public MoveMessage move(Long gameId, MoveRequest move, String username) {
        var gameOpt = getGameById(gameId);
        if (gameOpt.isEmpty()) {
            gamePublisher.getGame(gameId);
            var msg = MoveMessage.rejected(move.getMove(), username, "Game not loaded, please wait!");
            simpMessagingTemplate.convertAndSend("/topic/game/" + gameId, msg);
            return msg;
        }
        var game = gameOpt.get();

        if (!game.isUserInGame(username)) {
            var msg = MoveMessage.rejected(move.getMove(), username, "Cannot move!");
            simpMessagingTemplate.convertAndSend("/topic/game/" + gameId, msg);
            return msg;
        }
        if (game.isFinished()) {
            var msg =  MoveMessage.rejected(move.getMove(), username, "Game is finished!");
            simpMessagingTemplate.convertAndSend("/topic/game/" + gameId, msg);
            return msg;
        }

        if (game.isBlocked() || !canPlayerMove(game, move, username)) {
            var msg = MoveMessage.rejected(move.getMove(), username, "Cannot move now!");
            simpMessagingTemplate.convertAndSend("/topic/game/" + gameId, msg);
            return msg;
        }
        game.setBlocked(true);
        repository.save(game);

        movePublisher.sendMove(game, move.getMove());

        return MoveMessage.of(move.getMove(), username);
    }

    public Optional<GameState> getGameById(Long id) {
        return repository.findById(id);
    }

    private boolean canPlayerMove(GameState game, MoveRequest move, String username) {
        return ((username.equals(game.getUsername1()) && game.isFirstUserTurn()) ||
                (username.equals(game.getUsername2()) && game.isSecondUserTurn()));
    }

    public void loadGame(StateEvent event) {
        if (event.isError()) {
            logger.debug("Error in state event for game {}", event.getId());
            var msg = new GameMessage();
            msg.setError(event.getErrorMessage());
            simpMessagingTemplate.convertAndSend("/topic/board/" + event.getId(), msg);
            return;
        }
        if (event.isFinished()) {
            logger.debug("Finished state event for game {}", event.getId());
            var msg = new GameMessage();
            msg.setError("Game is already finished!");
            simpMessagingTemplate.convertAndSend("/topic/board/" + event.getId(), msg);
            return;
        }

        logger.debug("Adding state for game {} to Redis", event.getId());
        var game = new GameState();
        game.setId(event.getId());
        game.setUsername1(event.getUsername1());
        game.setUsername2(event.getUsername2());
        game.setUser2AI(event.isUser2AI());

        game.setFirstUserStarts(event.isFirstUserStarts());
        game.setFirstUserTurn(event.isFirstUserTurn());

        game.setCurrentState(event.getCurrentState());
        game.setAiType(event.getAiType());
        repository.save(game);
        logger.debug("Sending state of game {} to websocket topic", event.getId());
        var msg = GameMessage.of(game);
        simpMessagingTemplate.convertAndSend("/topic/board/" + game.getId(), msg);
        if (game.aiTurn()) {
            logger.debug("Asking AI engine for move in game {}", event.getId());
            movePublisher.sendAIEvent(game);
        }
    }

    public void doMakeMove(EngineMoveEvent event) {
        var game = getGameById(event.getGameId()).orElseThrow();
        if (!event.isLegal()) {
            game.setBlocked(false);
            simpMessagingTemplate.convertAndSend(
                    "/topic/game/" + game.getId(),
                    MoveMessage.rejected(
                            event.getMove(),
                            game.getCurrentPlayer(),
                            "Move is illegal!"
                    )
            );
            repository.save(game);
            return;
        }

        if (event.isFinished()) {
            game.setFinished(true);
            if (game.isFirstUserTurn()) {
                game.setWon(true);
            } else {
                game.setLost(true);
            }
        }
        var msg = MoveMessage.of(event.getMove(), game.getCurrentPlayer());
        if (game.isFinished()) {
            msg.setFinished(true);
            msg.setWon(game.isWon());
            msg.setWinner(game.getWinner().orElse(null));
            repository.deleteById(game.getId());
        } else {
            game.nextPlayer();
            game.setBlocked(false);
            repository.save(game);
        }
        statePublisher.publish(game, event.getMove());

        simpMessagingTemplate.convertAndSend("/topic/game/" + game.getId(), msg);
        if (!game.isFinished() && game.aiTurn()) {
            movePublisher.sendAIEvent(game);
        }
    }
}
