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
}
